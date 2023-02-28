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
        if let Some(next_pattern) = route.next() {
            if let Some(child) = self.child_matching_pattern_mut(&next_pattern) {
                child.register(route, handle_func)
            } else {
                let mut child = Node::new(next_pattern);
                child.register(route, handle_func);
                self.children.push(child)
            }
        } else {
            self.handle_func = Some(handle_func)
        }
    }
    pub fn child_matching_pattern_mut(&mut self, pattern: &Pattern) -> Option<&mut Self> {
        for child in &mut self.children {
            if pattern.matches(&child.pattern) {
                return Some(child)
            }
        }
        None
    }

    // pub fn radixize(&mut self) {
    //     match self.children.len() {
    //         0 => (),
    //         1 => {
    //             
    //         },
    //         _ => {
// 
    //         },
    //     }
    // }
}
