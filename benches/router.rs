#![feature(test)]
extern crate test;
use test::Bencher;

use experiment::router::{
    Router, TrieTreeRouter, RegexSetRouter2,
};

mod setup {
    use experiment::router::{Request, Response, Handler, Method, Method::*};

    pub const TEST_ROUTES_SIZE: usize = 16;
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
    pub const TEST_CASES: &'static [&'static str] = &[
        "GET /",
        "GET /hc",
        "GET /api/users/1",
        "POST /api/users/100", 
        "POST /api/users",
        "PATCH /api/users", 
        "PATCH /api/users/42",
        "GET /api/v2/users/1000",
        "GET /api/subtasks/2",
        "POST /api/subtasks/42/314",
        "POST /api/subtasks//314",
        "GET /hc",
        "GET /hc/2",
        "GET /api/users/1000",
        "POST /api/users", 
        "POST /api/users/",
        "PATCH /api/users", 
        "PATCH /api/users/42",
        "GET /api/tasks/1000",
        "GET /api/tasks",
        "POST /api/tasks",
        "POST /api/tasks/42",
    ];
}

#[bench] // 122,600 ns/iter (+/- 1,756)
fn trie_tree_router(b: &mut Bencher) {
    let router = TrieTreeRouter::new(setup::TEST_ROUTES());
    b.iter(|| for _ in 0..100 {
        for case in setup::TEST_CASES {
            let result = <TrieTreeRouter as Router<{setup::TEST_ROUTES_SIZE}>>::search(&router, &case);
            match result {
                Some(_) => println!("Found"),
                None    => println!("Not found"),
            }
        }
    })
}

#[bench] // 1,225,836 ns/iter (+/- 8,980)
fn regex_set_router(b: &mut Bencher) {
    let router = RegexSetRouter2::new(setup::TEST_ROUTES());
    b.iter(|| for _ in 0..100 {
        for case in setup::TEST_CASES {
            let result = router.search(&case);
            match result {
                Some(_) => println!("Found"),
                None    => println!("Not found"),
            }
        }
    })
}
