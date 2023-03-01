pub struct RadixTreeRouter {
    GET: RadixNode,
    POST: RadixNode,
    PATCH: RadixNode,
    DELETE: RadixNode,
} impl RadixTreeRouter {
    pub fn from_radix_tree(tree: RadixTree) -> Self {
        Self {
            GET: RadixNode::from_node(tree.GET),
            POST: RadixNode::from_node(tree.POST),
            PATCH: RadixNode::from_node(tree.PATCH),
            DELETE: RadixNode::from_node(tree.DELETE),
        }
    }
}
struct RadixNode {
    patterns: &'static [Pattern],
    handle_func: Option<HandleFunc>,
    children: Vec<RadixNode>,
} impl RadixNode {
    fn from_node(node: Node) -> Self {
        Self {
            patterns:    Box::leak(node.patterns.into_boxed_slice()),
            handle_func: node.handle_func,
            children:    node.children.into_iter().map(|n| RadixNode::from_node(n)).collect(),
        }
    }
}

const _: () = {
    impl<const N: usize> Router<N> for RadixTreeRouter {
        fn new(handlers: [Handler; N]) -> Self {
            let tree = RadixTree::new(handlers);
            Self::from_radix_tree(tree)
        }
        fn search<'buf>(&self, request_line: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
            let (method, path) = request_line.split_once(' ').unwrap();
            match method {
                "GET" => self.GET.search(path, vec![]),
                "POST" => self.POST.search(path, vec![]),
                "PATCH" => self.PATCH.search(path, vec![]),
                "DELETE" => self.DELETE.search(path, vec![]),
                _ => return None
            }
        }
    }

    impl<'buf> RadixNode {
        fn search(&self,
            mut path:   &'buf str,
            mut params: Vec<&'buf str>,
        ) -> Option<(&HandleFunc, Vec<&'buf str>)> {
            for pattern in self.patterns {
                if path.is_empty() {return None}
                match pattern {
                    Pattern::Nil => break,
                    Pattern::Str(s) => path = path.strip_prefix(s)?,
                    Pattern::Param => {
                        match path[1..].find('/') {
                            Some(len) => {
                                params.push(&path[1 .. 1+len]);
                                path = &path[1+len..]
                            },
                            None => {
                                params.push(&path[1..]);
                                path = ""
                            },
                        }
                    },
                }
            }

            if path.is_empty() {
                Some(((&self).handle_func.as_ref()?, params))
            } else {
                self.matchable_child(path)?.search(path, params)
            }
        }

        fn matchable_child(&self, current_path: &str) -> Option<&Self> {
            for child in &self.children {
                match child.patterns.first()? {
                    Pattern::Nil => unreachable!(),
                    Pattern::Param =>  return Some(child),
                    Pattern::Str(s) => if current_path.starts_with(s) {return Some(child)}
                }
            }
            None
        }
    }
};




mod test;
mod range_trie_tree;

use super::{HandleFunc, Handler, Router};
use self::range_trie_tree::RangeTrieTree;

pub struct RadixTree {
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
#[derive(PartialEq, Eq)]
enum Pattern {
    Str(&'static str),
    Param,
    Nil,
}

impl RadixTree {
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
}
impl Node {
    pub(super) fn from_trie(mut node: range_trie_tree::Node) -> Self {
        let mut patterns = vec![node.pattern.clone()];
        (node, patterns) = Self::merge_single_child(node, patterns);

        node.children.sort_by_key(|n| n.pattern.clone());

        Self {
            patterns:     patterns.into_iter().map(|p| Pattern::from(p)).collect(),
            handle_func:  node.handle_func,
            children:     node.children.into_iter().map(|n| Self::from_trie(n)).collect()
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
