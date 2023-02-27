use crate::router::HandleFunc;
use super::{route::Route, pattern::Pattern};

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
    fn new(pattern: Pattern) -> Self {
        Self {
            pattern,
            handle_func: None,
            children:    vec![],
        }
    }
    pub fn register(&mut self, route: &mut Route, handle_func: HandleFunc) {
        if let Some(next) = route.next() {

            todo!()
            
        } else {
            self.handle_func = Some(handle_func)
        }
    }
}
