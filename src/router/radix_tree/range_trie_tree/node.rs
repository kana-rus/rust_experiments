use crate::router::{HandleFunc, radix_tree::patterns::Pattern};
use super::route::Route;

pub struct Node {
    pub pattern:     Pattern,
    pub handle_func: Option<HandleFunc>,
    pub children:    Vec<Node>,
}

impl Node {
    pub fn root() -> Self {
        Self {
            pattern:     Pattern::Nil,
            handle_func: None,
            children:    vec![],
        }
    }
    pub fn register(&mut self, route: Route, handle_func: HandleFunc) {
        
    }
}
