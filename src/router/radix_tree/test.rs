#![cfg(test)]
use std::fmt::Debug;
use std::ops::Range;
use crate::router::{HandleFunc, Response, Router};
use super::range_trie_tree::{Node as TrieNode, Pattern as TriePattern, Section as TrieSection};
use super::super::radix_tree::{RadixTreeRouter, Node as RadixNode, Pattern as RadixPattern};

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
#{{ patterns: {:?}, handle_func: {}, children: {:#?} }}",
                self.patterns,
                if self.handle_func.is_some() {"Some"} else {"None"},
                self.children,
            )
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
};

const ROUTE_1: &'static str = "/api/v2/users";
const ROUTE_2: &'static str = "/api/tasks/completed";
const ROUTE_3: &'static str = "/api/users/:id";

fn Trie(
    pattern:  TriePattern,
    handle_func: Option<HandleFunc>,
    children: Vec<TrieNode>,
) -> TrieNode {
    TrieNode {
        pattern,
        handle_func,
        children,
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
fn S3(range: Range<usize>) -> TriePattern {
    TriePattern::Section(
        TrieSection { route_str: ROUTE_3, range }
    )
}
fn H() -> Option<HandleFunc> {
    Some(Box::new(|_| Box::pin(async {Response::Ok(format!(""))})))
}

fn Radix(patterns: Vec<RadixPattern>, handle_func: Option<HandleFunc>, children: Vec<RadixNode>) -> RadixNode {
    RadixNode {
        patterns,
        handle_func,
        children,
    }
}

#[test]
fn radix_from_trie() {
    let trie = Trie(TriePattern::Nil, None, vec![
        Trie(S1(0..4), None, vec![
            Trie(S1(4..7), None, vec![
                Trie(S1(7..13), None, vec![])
            ]),
            Trie(S2(4..10), None, vec![
                Trie(S2(10..20), None, vec![])
            ])
        ]),
    ]);
    let radix = Radix(vec![RadixPattern::Str("/api")], None, vec![
        Radix(vec![RadixPattern::Str("/tasks/completed")], None, vec![]),
        Radix(vec![RadixPattern::Str("/v2/users")], None, vec![]),
    ]);
    assert_eq!(RadixNode::from_trie(trie), radix);


    let trie = Trie(TriePattern::Nil, None, vec![
        Trie(S1(0..4), None, vec![
            Trie(S3(4..10), None, vec![
                Trie(TriePattern::Param, H(), vec![])
            ]),
            Trie(S1(4..7), None, vec![
                Trie(S1(7..13), H(), vec![
                    Trie(TriePattern::Param, H(), vec![])
                ])
            ]),
        ])
    ]);
    let radix = Radix(vec![RadixPattern::Str("/api")], None, vec![
        Radix(vec![RadixPattern::Str("/users"), RadixPattern::Param], H(), vec![]),
        Radix(vec![RadixPattern::Str("/v2/users")], H(), vec![
            Radix(vec![RadixPattern::Param], H(), vec![])
        ]),
    ]);
    assert_eq!(RadixNode::from_trie(trie), radix);
}

#[test]
fn search_radix() {
    let GET = Radix(vec![RadixPattern::Str("/api")], None, vec![
        Radix(vec![RadixPattern::Str("/users")], H(), vec![
            Radix(vec![RadixPattern::Param], H(), vec![])
        ]),
        Radix(vec![RadixPattern::Str("/tasks")], None, vec![
            Radix(vec![RadixPattern::Param], H(), vec![])
        ])
    ]);
    let POST = Radix(vec![RadixPattern::Nil], None, vec![]);
    let PATCH = Radix(vec![RadixPattern::Nil], None, vec![]);
    let DELETE = Radix(vec![RadixPattern::Nil], None, vec![]);

    let router = RadixTreeRouter {
        GET: super::RadixNode::from_node(GET),
        POST: super::RadixNode::from_node(POST),
        PATCH: super::RadixNode::from_node(PATCH),
        DELETE: super::RadixNode::from_node(DELETE),
    };

    let assert_search_hit = |request_line| assert!(
        <RadixTreeRouter as Router>::search(&router, request_line).is_some(),
        "{request_line}"
    );
    let assert_search_not_hit = |request_line| assert!(
        <RadixTreeRouter as Router>::search(&router, request_line).is_none(),
        "{request_line}"
    );

    assert_search_not_hit("GET /");
    assert_search_hit("GET /api/users");
    assert_search_hit("GET /api/tasks/1");
    assert_search_not_hit("GET /api/tasks");
    assert_search_hit("GET /api/users/42");
    assert_search_not_hit("GET /api/tasks");
}
