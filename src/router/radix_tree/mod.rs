mod test;
mod range_trie_tree;

use super::{HandleFunc, Handler};
use self::range_trie_tree::RangeTrieTree;

pub struct RadixTreeRouter {
    GET: Node,
    POST: Node,
    PATCH: Node,
    DELETE: Node,
}
pub(super) struct Node {
    patterns:     Vec<Pattern>,
    handle_func:  Option<HandleFunc>,
    children:     Vec<Node>,
}
enum Pattern {
    Str(&'static str),
    Param,
    Nil,
}

impl RadixTreeRouter {
    pub fn new<const N: usize>(handlers: [Handler; N]) -> Self {
        let mut trie_tree = RangeTrieTree::new();
        for Handler { method, route:route_str, proc:handle_func } in handlers {
            trie_tree.register(method, route_str, handle_func)
        }
        Self {
            GET: Node::from_trie(trie_tree.GET),
            POST: Node::from_trie(trie_tree.POST),
            PATCH: Node::from_trie(trie_tree.PATCH),
            DELETE: Node::from_trie(trie_tree.DELETE),
        }
    }
    pub fn search<'buf>(&self, path: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
        todo!()
    }
}
impl Node {
    pub(super) fn from_trie(mut node: range_trie_tree::Node) -> Self {
        let mut patterns = vec![node.pattern.clone()];
        (node, patterns) = Self::merge_single_child(node, patterns);
        Self {
            patterns:     patterns.into_iter().map(|p| Pattern::from(p)).collect(),
            handle_func:  node.handle_func,
            children:     node.children.into_iter().map(|n| Self::from_trie(n)).collect(),
        }
    }

    fn merge_single_child(
        mut node:     range_trie_tree::Node,
        mut patterns: Vec<range_trie_tree::Pattern>,
    ) -> (
        range_trie_tree::Node,
        Vec<range_trie_tree::Pattern>,
    ) {
        let this_pattern = patterns.last_mut().unwrap();

        if node.children.len() == 1
        && node.handle_func.is_none() {
            let child = node.children.pop().unwrap();
            let child_pattern = child.pattern.clone();

            if this_pattern.is_section() && child.pattern.is_section() {
                this_pattern.merge_sections(child_pattern)
            } else if this_pattern.is_nil() {
                *this_pattern = child_pattern
            } else {
                patterns.push(child_pattern)
            }

            node.children = child.children;
            node.handle_func = child.handle_func;
            Self::merge_single_child(node, patterns)
        } else {
            (node, patterns)
        }
    }
}

const _: (/* Pattern impls */) = {
    impl From<range_trie_tree::Pattern> for Pattern {
        fn from(pattern: range_trie_tree::Pattern) -> Self {
            match pattern {
                range_trie_tree::Pattern::Nil => Self::Nil,
                range_trie_tree::Pattern::Param => Self::Param,
                range_trie_tree::Pattern::Section(section) => Self::Str(section.read_str()),
            }
        }
    }
};
