use regex::{RegexSet, Regex};
use crate::router::Method;

use super::{Handler, Router};


pub struct RegexSetRouter1<'router, const N: usize> {
    GET: Node<'router, N>,
    POST: Node<'router, N>,
    PATCH: Node<'router, N>,
    DELETE: Node<'router, N>,
}
struct Node<'router, const N: usize> {
    routes:   RegexSet,
    handlers: [Option<Handler<'router>>; N],
}
const _: () = {
    impl<'router, const N: usize> Router<'router, N> for RegexSetRouter1<'router, N> {
        fn register(&mut self, methods: [Method; N], routes: [&'static str; N], handlers: [Handler<'router>; N]) {
            
        }
        fn search<'buf>(&'router self, request_line: &'buf str) -> Option<(&'router Handler, Vec<&'buf str>)> {
            
        }
    }
};


pub struct RegexSetRouter2<'router, const N: usize> {
    routes:   RegexSet,
    handlers: [Handler<'router>; N],
}
const _: () = {
    impl<'router, const N: usize> Router<'router, N> for RegexSetRouter2<'router, N> {
        fn register(&mut self, methods: [Method; N], routes: [&'static str; N], handlers: [Handler<'router>; N]) {
            let mut regex_routes = Vec::with_capacity(N);
            let param_pattern = Regex::new(":[a-zA-Z][_a-zA-Z0-9]*/").unwrap();
            for i in 0..N {
                let method_name = match methods[i] {
                    Method::GET => "GET",
                    Method::POST => "POST",
                    Method::PATCH => "PATCH",
                    Method::DELETE => "DELETE",
                };
                let regex_route_str = param_pattern.replace_all(routes[i], "([_a-zA-Z0-9]+)/");
                regex_routes.push(format!("{method_name} {regex_route_str}"))
            }
            self.routes = RegexSet::new(regex_routes).unwrap()
        }

        fn search<'buf>(&'router self, request_line: &'buf str) -> Option<(&'router Handler, Vec<&'buf str>)> {
            let matched = self.routes.matches(request_line)
                .into_iter()
                .last()?;
            Some((&self.handlers[matched], ))
        }
    }
};
