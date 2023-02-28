#![cfg(test)]
use std::ops::Range;

use crate::router::{HandleFunc, Response, radix_tree::{RadixTreeRouter, self}};
use super::range_trie_tree;

const ROUTE_1: &'static str = "/api/v2/users";
const ROUTE_2: &'static str = "/api/tasks/completed";

fn Node(
    pattern:  range_trie_tree::Pattern,
    children: Vec<range_trie_tree::Node>,
) -> range_trie_tree::Node {
    range_trie_tree::Node {
        pattern,
        children,
        handle_func: None,
    }
}
fn S1(range: Range<usize>) -> range_trie_tree::Pattern {
    range_trie_tree::Pattern::Section(
        range_trie_tree::Section { route_str: ROUTE_1, range }
    )
}
fn S2(range: Range<usize>) -> range_trie_tree::Pattern {
    range_trie_tree::Pattern::Section(
        range_trie_tree::Section { route_str: ROUTE_2, range }
    )
}
fn H() -> Option<HandleFunc> {
    Some(Box::new(|_| Box::pin(async {Response::Ok(format!(""))})))
}

#[test]
fn radix_from_trie() {
    use range_trie_tree::Pattern::*;
    let trie = Node(Nil, vec![
        Node(S1(1..4), vec![
            Node(S1(5..7), vec![
                Node(S1(8..13), vec![])
            ]),
            Node(S2(5..10), vec![
                Node(S2(11..20), vec![])
            ])
        ]),
    ]);

    let radix = radix_tree::Node::from_trie(trie);
    
}
