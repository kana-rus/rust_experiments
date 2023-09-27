use std::str::Split;
use super::{Method, Handler, Router, HandleFunc};


pub struct TrieTreeRouter {
    GET: Node,
    POST: Node,
    PATCH: Node,
    DELETE: Node,
} impl TrieTreeRouter {
    fn new() -> Self {
        Self {
            GET: Node::root(),
            POST: Node::root(),
            PATCH: Node::root(),
            DELETE: Node::root(),
        }
    }
}
struct Node {
    pattern: Pattern,
    handler: Option<HandleFunc>,
    children: Vec<Node>,
} impl Node {
    fn root() -> Self {
        Self {
            pattern:  Pattern::Nil,
            handler:  None,
            children: Vec::new(),
        }
    }
    fn new(section: &'static str) -> Self {
        Self {
            pattern:  Pattern::new(section),
            handler:  None,
            children: Vec::new(),
        }
    }
    fn matching_child<'path>(&self, section: &'path str) -> Option<&Self> {
        for child in &self.children {
            if child.pattern.matches(section) {
                return Some(child)
            }
        }
        None
    }
    fn matching_child_mut<'path>(&mut self, section: &'path str) -> Option<&mut Self> {
        for child in &mut self.children {
            if child.pattern.matches(section) {
                return Some(child)
            }
        }
        None
    }
}
#[derive(PartialEq, Debug)]
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
            Self::Param => !path_section.is_empty(),
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
        match path_str {
            "/" => {let mut s = "".split('/'); s.next(); Self(s)}//empty iterator
            _ => Self(path_str
                .trim_start_matches('/')
                .trim_end_matches('/')
                .split('/')
            )
        }
    }
} impl<'buf> Iterator for Path<'buf> {
    type Item = &'buf str;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}


const _: () = {
    impl Router for TrieTreeRouter {
        fn new<const N: usize>(handlers: [Handler; N]) -> Self {
            let mut this = Self::new();
            for Handler { method, route, proc } in handlers {
                match method {
                    Method::GET => this.GET.register(route, proc),
                    Method::POST => this.POST.register(route, proc),
                    Method::PATCH => this.PATCH.register(route, proc),
                    Method::DELETE => this.DELETE.register(route, proc),
                }
            }
            this   
        }
        fn search<'buf>(&self, request_line: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
            let (method, path) = request_line.split_once(' ').unwrap();
            match method {
                "GET" => self.GET.search(path),
                "POST" => self.POST.search(path),
                "PATCH" => self.PATCH.search(path),
                "DELETE" => self.DELETE.search(path),
                _ => return None
            }
        }
    }
};

impl Node {
    fn register(&mut self, route: &'static str, handler: HandleFunc) {
        let mut route: Path<'static> = Path::new(route);
        self._register(&mut route, handler)
    }
    fn search<'buf>(&self, path: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
        const PARAMS_CAPACITY: usize = 2;
        let mut path = Path::new(path);
        let params = Vec::with_capacity(PARAMS_CAPACITY);
        self._search(&mut path, params)
    }
    
    fn _register(&mut self, route: &mut Path<'static>, handler: HandleFunc) {
        if let Some(section) = route.next() {
            if let Some(child) = self.matching_child_mut(section) {
                child._register(route, handler)
            } else {
                let  mut child = Node::new(section);
                child._register(route, handler);
                self.children.push(child)
            }
        } else {
            self.handler = Some(handler)
        }
    }
    fn _search<'buf>(&self, path: &mut Path<'buf>, mut params: Vec<&'buf str>) -> Option<(&HandleFunc, Vec<&'buf str>)> {
        if let Some(section) = path.next() {
            let child = self.matching_child(section)?;
            if child.pattern.is_param() {params.push(section)}
            child._search(path, params)
        } else {
            Some(((self.handler.as_ref())?, params))
        }
    }
}


#[cfg(test)]
mod test {
    use std::fmt::Debug;
    use crate::router::{test::handler, Router, Method::*, TrieTreeRouter, trie_tree::Pattern::*, HandleFunc};

    #[allow(non_snake_case)]
    fn Handler() -> Option<HandleFunc> {
        Some(Box::new(|req| Box::pin(crate::router::test::handle_func(req))))
    }
    #[allow(non_snake_case)]
    fn Node(pattern: super::Pattern, handler: Option<HandleFunc>, children: Vec<super::Node>) -> super::Node {
        super::Node { pattern, handler, children }
    }

    const _: () = {
        impl PartialEq for TrieTreeRouter {
            fn eq(&self, other: &Self) -> bool {
                self.GET == other.GET &&
                self.POST == other.POST &&
                self.PATCH == other.PATCH &&
                self.DELETE == other.DELETE
            }
        }
        impl Debug for TrieTreeRouter {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "
GET:   {:?}
POST:  {:?}
PATCH: {:?}
DELTE: {:?}
",
                    self.GET,
                    self.POST,
                    self.PATCH,
                    self.DELETE,
                )
            }
        }

        /* in this test, I'll use only one handle func */
        impl PartialEq for super::Node {
            fn eq(&self, other: &Self) -> bool {
                self.pattern == other.pattern
                && self.children == other.children
                && match &self.handler {
                    Some(_) => other.handler.is_some(),
                    None    => other.handler.is_none(),
                }
            }
        }
        impl Debug for super::Node {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "#{{ pattern: {:?}, handler: {}, children: {:?} }}",
                    self.pattern,
                    if self.handler.is_some() {"Some"} else {"None"},
                    self.children,
                )
            }
        }
    };

    #[test]
    fn trie_tree_new() {
        assert_eq!(
            <TrieTreeRouter as Router>::new([
                handler(GET, "/")
            ]), TrieTreeRouter {
                GET: Node(Nil, Handler(), vec![]),
                POST: Node(Nil, None, vec![]),
                PATCH: Node(Nil, None, vec![]),
                DELETE: Node(Nil, None, vec![]),
            }
        );
        assert_eq!(
            <TrieTreeRouter as Router>::new([
                handler(GET, "/api")
            ]), TrieTreeRouter {
                GET: Node(Nil, None, vec![
                    Node(Str("api"), Handler(), vec![])
                ]),
                POST: Node(Nil, None, vec![]),
                PATCH: Node(Nil, None, vec![]),
                DELETE: Node(Nil, None, vec![]),
            }
        );
        assert_eq!(
            <TrieTreeRouter as Router>::new([
                handler(GET, "/api/users"),
                handler(GET, "/api/users/:id"),
            ]), TrieTreeRouter {
                GET: Node(Nil, None, vec![
                    Node(Str("api"), None, vec![
                        Node(Str("users"), Handler(), vec![
                            Node(Param, Handler(), vec![])
                        ])
                    ])
                ]),
                POST: Node(Nil, None, vec![]),
                PATCH: Node(Nil, None, vec![]),
                DELETE: Node(Nil, None, vec![]),
            }
        );
        assert_eq!(
            <TrieTreeRouter as Router>::new([
                handler(GET, "/api/users/:id"),
                handler(POST, "/api/users"),
                handler(GET, "/api/tasks/completed/:user_id"),
                handler(GET, "/api/tasks/all/:user_id"),
            ]), TrieTreeRouter {
                GET: Node(Nil, None, vec![
                    Node(Str("api"), None, vec![
                        Node(Str("users"), None, vec![
                            Node(Param, Handler(), vec![])
                        ]),
                        Node(Str("tasks"), None, vec![
                            Node(Str("completed"), None, vec![
                                Node(Param, Handler(), vec![])
                            ]),
                            Node(Str("all"), None, vec![
                                Node(Param, Handler(), vec![])
                            ])
                        ])
                    ])
                ]),
                POST: Node(Nil, None, vec![
                    Node(Str("api"), None, vec![
                        Node(Str("users"), Handler(), vec![])
                    ])
                ]),
                PATCH: Node(Nil, None, vec![]),
                DELETE: Node(Nil, None, vec![]),
            }
        );
    }

    #[test]
    fn trie_tree_search() {
        fn assert_search<const N: usize>(request_line: &'static str, routes: [(crate::router::Method, &'static str); N], expect_params: Option<&'static [&'static str]>) {
            match <TrieTreeRouter as Router>::search(
                &<TrieTreeRouter as Router>::new(routes.map(|(method, route)| crate::router::test::handler(method, route))),
                request_line
            ) {
                None              => assert_eq!(expect_params, None),
                Some((_, params)) => assert_eq!(Some(params), expect_params.map(|array| array.to_vec())),
            }
        }

        assert_search("GET /", [
            (GET, "/")
        ], Some(&[]));

        assert_search("GET /api", [
            (GET, "/api")
        ], Some(&[]));
        assert_search("GET /", [
            (GET, "/api")
        ], None);
        
        assert_search("GET /api/users/1", [
            (GET, "/api/users"),
            (GET, "/api/users/:id"),
        ], Some(&["1"]));
        assert_search("GET /api/users", [
            (GET, "/api/users"),
            (GET, "/api/users/:id"),
        ], Some(&[]));
        
        assert_search("GET /api/users/42", [
            (GET, "/api/users/:id"),
            (POST, "/api/users"),
            (GET, "/api/tasks/completed/:user_id"),
            (GET, "/api/tasks/all/:user_id"),
        ], Some(&["42"]));
        assert_search("POST /api/users/42", [
            (GET, "/api/users/:id"),
            (POST, "/api/users"),
            (GET, "/api/tasks/completed/:user_id"),
            (GET, "/api/tasks/all/:user_id"),
        ], None);
        assert_search("GET /api/tasks/all/user_id", [
            (GET, "/api/users/:id"),
            (POST, "/api/users"),
            (GET, "/api/tasks/completed/:user_id"),
            (GET, "/api/tasks/all/:user_id"),
        ], Some(&["user_id"]));

        assert_search("GET /api/v2/users/2", [
            (GET, "/api/:api_version/users/:id"),
        ], Some(&["v2", "2"]))
        
    }
}
