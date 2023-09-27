/* ---

^/(?:
    help$()
    |
    ([^/]+)/(?:
        followe(?:
            es$()
            |
            rs$()
        )
        |
        posts(?:
            $()
            |
            /([^/]+)(?:
                $()
                |
                /likes$()
            )
        )
    )
)

--- */

use std::mem::MaybeUninit;

use regex::Regex;
use crate::router::{Request, Response, Method};

use super::{Handler, Router, HandleFunc};


pub struct SingleRegexRouter {
    GET: Matcher,
    POST: Matcher,
    PATCH: Matcher,
    DELETE: Matcher,
}
struct Matcher {
    routes:   Regex,
    handlers: Vec<MaybeUninit<HandleFunc>>,
}


#[test] fn how_regex_works() {
    let r = Regex::new("/api/subtasks/([^/]+)(?:$()|/([^/]+)$())").unwrap();
    let params = |path: &'static str| {
        let mut matches = r.captures(path).unwrap()
            .iter()
            .skip(1)
            .filter_map(|om| om.map(|m| m.as_str()))
            .collect::<Vec<_>>();
        matches.pop();
        matches
    };

    assert_eq!(params("/api/subtasks/42"), vec!["42"]);
    assert_eq!(params("/api/subtasks/42/314"), vec!["42", "314"]);
}

#[test] fn test_single_regex_router() {
    let srr = SingleRegexRouter::new([]);

    assert!(srr.search("GET /").is_some_and(|(_, params)| params == Vec::<&str>::new()));
    assert!(srr.search("GET /hc").is_some_and(|(_, params)| params == Vec::<&str>::new()));
    assert!(srr.search("GET /hc/2").is_none());
    assert!(srr.search("GET /api/users").is_some_and(|(_, params)| params == Vec::<&str>::new()));
    assert!(srr.search("GET /api/users/").is_none());
    assert!(srr.search("GET /api/users/42").is_some_and(|(_, params)| params == vec!["42"]));
    assert!(srr.search("GET /api/users//").is_none());
    assert!(srr.search("GET /api/tasks").is_some_and(|(_, params)| params == Vec::<&str>::new()));
    assert!(srr.search("GET /api/tasks/42").is_some_and(|(_, params)| params == vec!["42"]));
    assert!(srr.search("GET /api/").is_none());
    assert!(srr.search("GET /api/subtasks").is_none());
    assert_eq!(srr.search("GET /api/subtasks/42").unwrap().1, vec!["42"]);
    assert_eq!(srr.search("GET /api/subtasks/42/314").unwrap().1, vec!["42", "314"]);
    assert!(srr.search("GET /api/v2").is_none());
    assert!(srr.search("GET /api/v2/tasks").is_some_and(|(_, params)| params == Vec::<&str>::new()));
    assert!(srr.search("GET /api/v2/tasks/").is_none());
    assert!(srr.search("GET /api/v2/tasks/314").is_some_and(|(_, params)| params == vec!["314"]));
}


impl Router for SingleRegexRouter {
    /// ===== THIS IS ONLY FOR TEST IN /benches/router.rs =====
    /// 
    /// ```
    /// pub fn TEST_ROUTES() -> [Handler; TEST_ROUTES_SIZE] {
    ///     [
    ///         handler(GET,    "/"),
    ///         handler(GET,    "/hc"),
    ///         handler(GET,    "/api/users"),
    ///         handler(GET,    "/api/users/:id"),
    ///         handler(PATCH,  "/api/users/:id"),
    ///         handler(DELETE, "/api/users/:id"),
    ///         handler(GET,    "/api/tasks"),
    ///         handler(GET,    "/api/tasks/:user_id"),
    ///         handler(POST,   "/api/tasks"),
    ///         handler(GET,    "/api/v2/users/:id"),
    ///         handler(POST,   "/api/v2/users"),
    ///         handler(GET,    "/api/subtasks/:user_id"),
    ///         handler(POST,   "/api/subtasks/:user_id/:id"),
    ///         handler(DELETE, "/api/subtasks/:user_id"),
    ///         handler(PATCH,  "/api/subtasks/:user_id/:id"),
    ///         handler(GET,    "/api/subtasks/:user_id/:id"),
    ///         handler(GET,    "/api/v2/tasks/:id"),
    ///         handler(GET,    "/api/v2/tasks"),
    ///     ]
    /// }
    /// ```
    /// 
    /// TODO: implement creation process
    /// 
    fn new<const N: usize>(_: [Handler; N]) -> Self {
        fn new_handle_func() -> HandleFunc {Box::new(move |req| Box::pin(handle_func(req)))}
        async fn handle_func(Request { method, path }: Request) -> Response {
            Response::Ok(format!("got `{} {path}`", match method {
                Method::GET => "GET",
                Method::POST => "POST",
                Method::PATCH => "PATCH",
                Method::DELETE => "DELETE",
            }))
        }

        Self {
            GET: Matcher {
                routes: Regex::new(
                    //     1      2                  3   4      5            6    7      8                9      10          11   12     13             14        15  16     17
                    "^/(?:$()|hc$()|api/(?:users(?:$()|/([^/]+)$())|tasks(?:$()|/([^/]+)$())|v2/(?:users/([^/]+)$()|tasks(?:$()|/([^/]+)$()))|subtasks/([^/]+)(?:$()|/([^/]+)$())))"
                ).unwrap(), handlers: vec![
                    MaybeUninit::new(new_handle_func()), // 1
                    MaybeUninit::new(new_handle_func()), // 2
                    MaybeUninit::new(new_handle_func()), // 3
                    MaybeUninit::uninit(),               // 4
                    MaybeUninit::new(new_handle_func()), // 5
                    MaybeUninit::new(new_handle_func()), // 6
                    MaybeUninit::uninit(),               // 7
                    MaybeUninit::new(new_handle_func()), // 8
                    MaybeUninit::uninit(),               // 9
                    MaybeUninit::new(new_handle_func()), // 10
                    MaybeUninit::new(new_handle_func()), // 11
                    MaybeUninit::uninit(),               // 12
                    MaybeUninit::new(new_handle_func()), // 13
                    MaybeUninit::uninit(),               // 14
                    MaybeUninit::new(new_handle_func()), // 15
                    MaybeUninit::uninit(),               // 16
                    MaybeUninit::new(new_handle_func()), // 17
                ],
            },
            POST: Matcher {
                routes: Regex::new(
                    "^/api/(?:tasks$()|v2/users$()|subtasks/([^/]+)/([^/]+)$())"
                ).unwrap(), handlers: vec![
                    MaybeUninit::new(new_handle_func()),
                    MaybeUninit::new(new_handle_func()),
                    MaybeUninit::uninit(),
                    MaybeUninit::uninit(),
                    MaybeUninit::new(new_handle_func()),
                ],
            },
            PATCH: Matcher {
                routes: Regex::new(
                    "^/api/users/(?:([^/]+)$()|subtasks/([^/]+)/([^/]+)$())"
                ).unwrap(), handlers: vec![
                    MaybeUninit::uninit(),
                    MaybeUninit::new(new_handle_func()),
                    MaybeUninit::uninit(),
                    MaybeUninit::uninit(),
                    MaybeUninit::new(new_handle_func()),
                ],
            },
            DELETE: Matcher {
                routes: Regex::new(
                    "^/api/(?:users/([^/]+)$()|subtasks/([^/]+)$())"
                ).unwrap(), handlers: vec![
                    MaybeUninit::uninit(),
                    MaybeUninit::new(new_handle_func()),
                    MaybeUninit::uninit(),
                    MaybeUninit::new(new_handle_func()),
                ],
            },
        }
    }
    #[inline] fn search<'buf>(&self, request_line: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
        match request_line.split_once(' ')? {
            ("GET", path) => self.GET.search(path),
            ("POST", path) => self.POST.search(path),
            ("PATCH", path) => self.PATCH.search(path),
            ("DELETE", path) => self.DELETE.search(path),
            _ => unreachable!()
        }
    }
}

const _: () = {
    impl Matcher {
        #[inline] fn search<'buf>(&self, path: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
            let captures = dbg!(self.routes.captures(path)?
                .iter()
                .skip(1)
                .map(|om| om.map(|m| m.as_str()))
                .collect::<Vec<_>>());

            let handle_func_index = dbg!(captures.iter().position(|os| os.is_some_and(|s| s == ""))).unwrap();
            let handle_func = unsafe {self.handlers[handle_func_index].assume_init_ref()};
            
            let params = dbg!({let mut matches = captures.into_iter().filter_map(|os| os).collect::<Vec<_>>(); matches.pop(/* "" */); matches});

            Some((handle_func, params))
        }
    }
};
