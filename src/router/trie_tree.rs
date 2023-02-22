use std::str::Split;
use super::{Method, Handler, Router};


pub struct TrieTreeRouter<'router> {
    GET: Node<'router>,
    POST: Node<'router>,
    PATCH: Node<'router>,
    DELETE: Node<'router>,
} impl<'router> TrieTreeRouter<'router> {
    fn new() -> Self {
        Self {
            GET: Node::root(),
            POST: Node::root(),
            PATCH: Node::root(),
            DELETE: Node::root(),
        }
    }
}

struct Node<'router> {
    pattern: Pattern,
    handler: Option<Handler<'router>>,
    chidlren: Vec<Node<'router>>,
} impl<'router> Node<'router> {
    fn root() -> Self {
        Self {
            pattern:  Pattern::Nil,
            handler:  None,
            chidlren: Vec::new(),
        }
    }
    fn new(section: &'static str) -> Self {
        Self {
            pattern:  Pattern::new(section),
            handler:  None,
            chidlren: Vec::new(),
        }
    }
    fn matching_child<'path>(&self, section: &'path str) -> Option<&Self> {
        for child in &self.chidlren {
            if child.pattern.matches(section) {
                return Some(child)
            }
        }
        None
    }
    fn matching_child_mut<'path>(&mut self, section: &'path str) -> Option<&mut Self> {
        for child in &mut self.chidlren {
            if child.pattern.matches(section) {
                return Some(child)
            }
        }
        None
    }
}

enum Pattern {
    Nil,
    Param,
    Str(&'static str),
} impl Pattern {
    fn new(path_section: &'static str) -> Self {
        if path_section.starts_with(':') {
            Self::Param
        } else {
            Self::Str(path_section)
        }
    }
    fn matches(&self, path_section: &str) -> bool {
        match self {
            Self::Nil => unreachable!(),
            Self::Param => path_section.starts_with(':'),
            Self::Str(string) => string == &path_section,
        }
    }
    fn is_param(&self) -> bool {
        match self {
            Self::Param => true,
            _ => false,
        }
    }
}


struct Path<'buf>(
    Split<'buf, char>
); impl<'buf> Path<'buf> {
    fn new(path_str: &'buf str) -> Self {
        Self(
            path_str
                .trim_start_matches('/')
                .trim_end_matches('/')
                .split('/')
        )
    }
} impl<'buf> Iterator for Path<'buf> {
    type Item = &'buf str;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}


const _: () = {
    impl<'router> Router<'router> for TrieTreeRouter<'router> {
        fn register(&mut self, method: super::Method, route: &'static str, handler: Handler<'router>) {
            match method {
                Method::GET => self.GET.register(route, handler),
                Method::POST => self.POST.register(route, handler),
                Method::PATCH => self.PATCH.register(route, handler),
                Method::DELETE => self.DELETE.register(route, handler),
            }
        }
        fn search<'buf>(&self, method: super::Method, path: &'buf str) -> Option<(&'router Handler, Vec<&'buf str>)> {
            match method {
                Method::GET => self.GET.search(path),
                Method::POST => self.POST.search(path),
                Method::PATCH => self.PATCH.search(path),
                Method::DELETE => self.DELETE.search(path),
            }
        }
    }
};

impl<'router> Node<'router> {
    fn register(&mut self, route: &'static str, handler: Handler<'router>) {
        let mut route: Path<'static> = Path::new(route);
        self._register(&mut route, handler)
    }
    fn search<'buf>(&'router self, path: &'buf str) -> Option<(&'router Handler<'router>, Vec<&'buf str>)> {
        let mut path = Path::new(path);
        let params = Vec::new();
        self._search(&mut path, params)
    }
    
    fn _register(&mut self, route: &mut Path<'static>, handler: Handler<'router>) {
        if let Some(section) = route.next() {
            if let Some(child) = self.matching_child_mut(section) {
                child._register(route, handler)
            } else {
                let  mut child = Node::new(section);
                child._register(route, handler);
                self.chidlren.push(child)
            }
        } else {
            self.handler = Some(handler)
        }
    }
    fn _search<'buf>(&'router self, path: &mut Path<'buf>, mut params: Vec<&'buf str>) -> Option<(&'router Handler, Vec<&'buf str>)> {
        if let Some(section) = path.next() {
            let child = self.matching_child(section)?;
            if child.pattern.is_param() {params.push(&section[1..])}
            child._search(path, params)
        } else {
            Some(((self.handler.as_ref())?, params))
        }
    }
}
