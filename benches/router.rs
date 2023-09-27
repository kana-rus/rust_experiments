#![feature(test)]
extern crate test;
use test::Bencher;

use experiment::router::{Router,
    // TrieTreeRouter,
    // TrieTreeRouterWithString,
    // RegexSetRouter2,
    // single_regex::SingleRegexRouter,
    radix_tree::{
        RadixTreeRouter,
        RadixTreeRouterWithStaticNodes,
        RadixTreeRouterWithVecPatterns,
    },
};

mod setup {
    use experiment::router::{Request, Response, Handler, Method, Method::*};

    pub const TEST_ROUTES_SIZE: usize = 18;
    pub(super) async fn handle_func(req: Request) -> Response {
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
    pub(super) fn handler(method: Method, route: &'static str) -> Handler {
        Handler {
            method,
            route,
            proc: Box::new(move |req| Box::pin(
                handle_func(req)
            )),
        }
    }
    #[allow(non_snake_case)]
    pub fn TEST_ROUTES() -> [Handler; TEST_ROUTES_SIZE] {
        [
            handler(GET,    "/"),
            handler(GET,    "/hc"),
            handler(GET,    "/api/users"),
            handler(GET,    "/api/users/:id"),
            handler(PATCH,  "/api/users/:id"),
            handler(DELETE, "/api/users/:id"),
            handler(GET,    "/api/tasks"),
            handler(GET,    "/api/tasks/:user_id"),
            handler(POST,   "/api/tasks"),
            handler(GET,    "/api/v2/users/:id"),
            handler(POST,   "/api/v2/users"),
            handler(GET,    "/api/subtasks/:user_id"),
            handler(POST,   "/api/subtasks/:user_id/:id"),
            handler(DELETE, "/api/subtasks/:user_id"),
            handler(PATCH,  "/api/subtasks/:user_id/:id"),
            handler(GET,    "/api/subtasks/:user_id/:id"),
            handler(GET,    "/api/v2/tasks/:id"),
            handler(GET,    "/api/v2/tasks"),
        ]
    }
    pub const TEST_CASES: &'static [&'static str] = &[
        "GET /",
        "GET /hc",
        "GET /api/users/1",
        "GET /api/users/100", 
        "GET /api/users",
        "GET /api/users//", 
        "GET /api/users/42",
        "GET /api/v2/users/1000",
        "GET /api/subtasks/2",
        "GET /api/subtasks/42/314",
        "GET /api/subtasks//314",
        "GET /hc/2",
        "GET /api/users/1000",
        "GET /api/users/v2/1000", 
        "GET /api/users/",
        "GET /api/users", 
        "GET /api/v2/users", 
        "GET /api/users/42",
        "GET /api/tasks/1000",
        "GET /api/tasks/v2",
        "GET /api/tasks",
        "GET /api/v2/tasks",
        "GET /api/tasks/42",
        "GET /api/v2/tasks/42",
    ];
}




macro_rules! benchmark {
    ($( $target:ident )*) => {$(
        #[bench]
        #[allow(non_snake_case)]
        fn $target(b: &mut Bencher) {
            let router = <$target as Router>::new(setup::TEST_ROUTES());
            b.iter(|| for _ in 0..314 {for case in setup::TEST_CASES {
                let result = <$target as Router>::search(&router, &case);
                match result {
                    Some(_) => println!("Found, params: "),
                    None    => println!("Not found"),
                }
            }})
        }
    )*};
} benchmark! {
    // TrieTreeRouter
    // TrieTreeRouterWithString
    RadixTreeRouter
    RadixTreeRouterWithStaticNodes
    RadixTreeRouterWithVecPatterns
    // RegexSetRouter2
    // SingleRegexRouter
}
