mod pattern;
mod route; use route::Route;
mod node; use node::Node;

use crate::router::{Method, HandleFunc};

pub struct RangeTrieTree {
    pub GET:    Node,
    pub POST:   Node,
    pub PATCH:  Node,
    pub DELETE: Node,
}

impl RangeTrieTree {
    pub fn new() -> Self {
        Self {
            GET:    Node::root(),
            POST:   Node::root(),
            PATCH:  Node::root(),
            DELETE: Node::root(),
        }
    }
    pub fn register(&mut self, method: Method, route_str: &'static str, handle_func: HandleFunc) {
        let mut route = Route::new(route_str);
        match method {
            Method::GET => &mut self.GET,
            Method::POST => &mut self.POST,
            Method::PATCH => &mut self.PATCH,
            Method::DELETE => &mut self.DELETE,
        }.register(&mut route, handle_func)
    }
}
