#![cfg(test)]
use std::fmt::Debug;
use std::ops::Range;
use crate::router::{HandleFunc, Response};
use super::range_trie_tree::{Node as TrieNode, Pattern as TriePattern, Section as TrieSection};
use super::super::radix_tree::{Node as RadixNode, Pattern as RadixPattern};

const _: () = {
    impl PartialEq for RadixNode {
        fn eq(&self, other: &Self) -> bool {
            self.patterns == other.patterns &&
            self.children == other.children && (
                (self.handle_func.is_some() && other.handle_func.is_some()) ||
                (self.handle_func.is_none() && other.handle_func.is_none())
            )
        }
    }
    impl Debug for RadixNode {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "
#{{patterns: {:?}, handle_func: {}, children: {:?}}}",
                self.patterns,
                if self.handle_func.is_some() {"Some"} else {"None"},
                self.children,
            )
        }
    }

    impl PartialEq for RadixPattern {
        fn eq(&self, other: &Self) -> bool {
            match self {
                Self::Nil => other.is_nil(),
                Self::Param => other.is_param(),
                Self::Str(s) => Some(s) == other.read_str()
            }
        }
    }
    impl Debug for RadixPattern {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
           match self {
                Self::Nil => write!(f, "Nil"),
                Self::Param => write!(f, "Param"),
                Self::Str(s) => write!(f, "`{s}`"),
            }
        }
    }

    impl RadixPattern {
        fn is_nil(&self) -> bool {
            match self {
                Self::Nil => true,
                _ => false,
            }
        }
        fn is_param(&self) -> bool {
            match self {
                Self::Param => true,
                _ => false,
            }
        }
        fn read_str(&self) -> Option<&&'static str> {
            match self {
                Self::Str(s) => Some(s),
                _ => None,
            }
        }
    }
};

const ROUTE_1: &'static str = "/api/v2/users";
const ROUTE_2: &'static str = "/api/tasks/completed";

fn Trie(
    pattern:  TriePattern,
    children: Vec<TrieNode>,
) -> TrieNode {
    TrieNode {
        pattern,
        children,
        handle_func: None,
    }
}
fn S1(range: Range<usize>) -> TriePattern {
    TriePattern::Section(
        TrieSection { route_str: ROUTE_1, range }
    )
}
fn S2(range: Range<usize>) -> TriePattern {
    TriePattern::Section(
        TrieSection { route_str: ROUTE_2, range }
    )
}
fn H() -> Option<HandleFunc> {
    Some(Box::new(|_| Box::pin(async {Response::Ok(format!(""))})))
}

fn Radix(patterns: Vec<RadixPattern>, children: Vec<RadixNode>) -> RadixNode {
    RadixNode {
        patterns,
        children,
        handle_func: None,
    }
}

#[test]
fn radix_from_trie() {
    let trie = Trie(TriePattern::Nil, vec![
        Trie(S1(1..4), vec![
            Trie(S1(5..7), vec![
                Trie(S1(8..13), vec![])
            ]),
            Trie(S2(5..10), vec![
                Trie(S2(11..20), vec![])
            ])
        ]),
    ]);

    let radix = Radix(vec![RadixPattern::Str("/api")], vec![
        Radix(vec![RadixPattern::Str("/v2/users")], vec![]),
        Radix(vec![RadixPattern::Str("/tasks/completed")], vec![])
    ]);

    assert_eq!(RadixNode::from_trie(trie), radix)
}
