//! RadixTreeRouter that Nodes' patterns is `Vec<Pattern>` instead of `&'static [Pattern]`

use crate::router::{HandleFunc, Router, Handler};
use super::{RadixTree, Pattern, Node};

/// RadixTreeRouter that Nodes' patterns is `Vec<Pattern>` instead of `&'static [Pattern]`
pub struct RadixTreeRouterWithVecPatterns {
    GET: RadixNode,
    POST: RadixNode,
    PATCH: RadixNode,
    DELETE: RadixNode,
} impl RadixTreeRouterWithVecPatterns {
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
    patterns: Vec<Pattern>,
    handle_func: Option<HandleFunc>,
    children: Vec<RadixNode>,
} impl RadixNode {
    fn from_node(node: Node) -> Self {
        Self {
            patterns:    node.patterns,
            handle_func: node.handle_func,
            children:    node.children.into_iter().map(|n| RadixNode::from_node(n)).collect(),
        }
    }
}

const _: () = {
    impl Router for RadixTreeRouterWithVecPatterns {
        fn new<const N: usize>(handlers: [Handler; N]) -> Self {
            let tree = RadixTree::new(handlers);
            Self::from_radix_tree(tree)
        }
        #[inline] fn search<'buf>(&self, request_line: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
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
            for pattern in &self.patterns {
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