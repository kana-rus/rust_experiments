#![allow(non_snake_case)]
use std::{pin::Pin, future::Future};

mod trie_tree; pub use trie_tree::TrieTreeRouter;
mod regex_set; pub use regex_set::{/*RegexSetRouter1, */ RegexSetRouter2};

pub trait Router<const N: usize> {
    fn new(handlers: [Handler; N]) -> Self;
    /// `request_line` は末尾の ` HTTP/1.1` を除いた `{method} {path}` の形を想定
    fn search<'buf>(&self, request_line: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)>;
}

pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
}

pub struct Handler {
    pub method: Method,
    pub route: &'static str,
    pub proc: HandleFunc,
}

#[allow(unused)]
pub struct Request {
    method: Method,
    path:   &'static str,
} impl Request {
    #[allow(unused)] /// just for test
    fn from(request_line: &'static str) -> Self {
        match request_line.split_once(' ').unwrap() {
            ("GET", path) => Self { method: Method::GET, path },
            ("POST", path) => Self { method: Method::POST, path },
            ("PATCH", path) => Self { method: Method::PATCH, path },
            ("DELETE", path) => Self { method: Method::DELETE, path },
            _ => unreachable!()
        }
    }
}

pub enum Response {
    Ok(String),
    Err(String),
}

pub type HandleFunc = Box<dyn
    Fn(Request) -> Pin<
        Box<dyn
            Future<Output=Response>
            + Send
        >
    > + Send + Sync
>;


#[cfg(test)]
mod test {
    use std::future::Future;
    use super::{Method, Method::*};
    use super::{TrieTreeRouter, RegexSetRouter2, Router, Handler, Request, Response};

    const TEST_ROUTES_SIZE: usize = 16;
    async fn handle_func(req: Request) -> Response {
        Response::Ok(format!("got `{} {}`",
            match req.method {
                GET => "GET",
                POST => "POST",
                PATCH => "PATCH",
                DELETE => "DELETE",
            },
            req.path
        ))
    }
    fn handler(method: Method, route: &'static str) -> Handler {
        Handler {
            method,
            route,
            proc: Box::new(move |req| Box::pin(
                handle_func(req)
            )),
        }
    }
    #[allow(non_snake_case)]
    fn TEST_ROUTES() -> [Handler; TEST_ROUTES_SIZE] {
        [
            handler(GET,    "/"),
            handler(GET,    "/hc"),
            handler(GET,    "/api/users/:id"),
            handler(POST,   "/api/users"),
            handler(PATCH,  "/api/users/:id"),
            handler(DELETE, "/api/users/:id"),
            handler(GET,    "/api/tasks/:user_id"),
            handler(POST,   "/api/tasks"),
            handler(GET,    "/api/v2/users/:id"),
            handler(POST,   "/api/v2/users"),
            handler(GET,    "/api/subtasks/:user_id"),
            handler(POST,   "/api/subtasks/:user_id/:id"),
            handler(DELETE, "/api/subtasks/:users/:id"),
            handler(PATCH,  "/api/users/subtasks/:user_id/:id"),
            handler(GET,    "/api/v2/tasks/:id"),
            handler(POST,   "/api/v2/tasks"),
        ]
    }

    struct Case {
        request: &'static str,
        expect:  Option<&'static str>,
    }
    fn assert_response<F: Future<Output = Response>>(response: F, expect: &Option<&'static str>, request: &str) {
        match async_std::task::block_on(response) {
            Response::Ok(body) => {
                assert_eq!(&Some(body.as_str()), expect, "in {request}");
            },
            Response::Err(_) => {
                assert!(expect.is_none(), "in {request}");
            },
        }
    }
    const TEST_CASES: &'static [Case] = &[
        Case {request: "GET /", expect: Some("got `GET /`")},
        Case {request: "GET /hc", expect: Some("got `GET /hc`")},
        Case {request: "GET /api/users/1", expect: Some("got `GET /api/users/1`")},
        Case {request: "POST /api/users/100", expect: None},
        Case {request: "POST /api/users", expect: Some("got `POST /api/users`")},
        Case {request: "PATCH /api/users", expect: None},
        Case {request: "PATCH /api/users/42", expect: Some("got `PATCH /api/users/42`")},
        Case {request: "GET /api/v2/users/1000", expect: Some("got `GET /api/users/1000`")},
        Case {request: "GET /api/subtasks/2", expect: Some("got `GET /api/subtasks/2`")},
        Case {request: "POST /api/subtasks/42/314", expect: Some("got `POST /api/subtasks/42/314`")},
        Case {request: "POST /api/subtasks//314", expect: None},
    ];

    #[test]
    fn trie_tree_router() {
        let router = TrieTreeRouter::new(TEST_ROUTES());
        for Case { request, expect } in TEST_CASES {
            match <TrieTreeRouter as Router<TEST_ROUTES_SIZE>>::search(&router, &request) {
                None                   => assert!(expect.is_none(), "in {request}"),
                Some((handle_func, _)) => assert_response(handle_func(Request::from(request)), expect, &request),
            }
        }
    }

    #[test]
    fn regex_set_router() {
        let router = RegexSetRouter2::new(TEST_ROUTES());
        for Case { request, expect } in TEST_CASES {
            match router.search(&request) {
                None                   => assert!(expect.is_none(), "in {request}"),
                Some((handle_func, _)) => assert_response(handle_func(Request::from(request)), expect, &request),
            }
        }
    }
}
