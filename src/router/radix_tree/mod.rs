mod patterns; use patterns::Patterns;
mod range_trie_tree;
use super::{HandleFunc, Method, Handler};

pub struct RadixTreeRouter {
    GET: Node,
    POST: Node,
    PATCH: Node,
    DELETE: Node,
}
struct Node {
    patterns: Patterns,
    handler:  Option<HandleFunc>,
    children: Vec<Node>,
}

impl RadixTreeRouter {
    fn new<const N: usize>(handlers: [Handler; N]) -> Self {
        let (
            mut GET,
            mut POST,
            mut PATCH,
            mut DELETE,
        ) = (
            Node::root(),
            Node::root(),
            Node::root(),
            Node::root(),
        );
        for Handler { method, route, proc } in handlers {
            match method {
                Method::GET => GET.register(route, proc),
                Method::POST => POST.register(route, proc),
                Method::PATCH => PATCH.register(route, proc),
                Method::DELETE => DELETE.register(route, proc),
            }
        }
        Self {
            GET: GET.into_radix(),
            POST: POST.into_radix(),
            PATCH: PATCH.into_radix(),
            DELETE: DELETE.into_radix(),
        }
    }
    fn search<'buf>(&self, path: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
        todo!()
    }
}
impl Node {
    fn root() -> Self {
        Self {
            patterns: Patterns::root(),
            handler:  None,
            children: vec![],
        }
    }

    fn into_radix(mut self) -> Self {
        self.radixize();
        self
    }
    fn radixize(&mut self) {
        match self.children.len() {
            0 => (),
            1 => {
                self.children[0].radixize();

            },
            _ => {
            },
        }
    }

    fn register(&mut self, route: &'static str, proc: HandleFunc) {

    }
}
