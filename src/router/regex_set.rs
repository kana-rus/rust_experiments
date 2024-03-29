use regex::{RegexSet, Regex};
use super::{Handler, Router, Method, HandleFunc};


// pub struct RegexSetRouter1<'router, const N: usize> {
//     GET: Node<'router, N>,
//     POST: Node<'router, N>,
//     PATCH: Node<'router, N>,
//     DELETE: Node<'router, N>,
// }
// struct Node<'router, const N: usize> {
//     regex_set: RegexSet,
//     routes:    [Option<Regex>; N],
//     handlers:  [Option<Handler>; N],
// } impl<'router, const N: usize> Node<'router, N> {
//     fn new(regex_strs: Vec<String>, handlers_for_this_method: Vec<Handler>) -> Self {
//         let mut routes = vec![None; N];
//         for (i, re_str) in regex_strs.iter().enumerate() {
//             routes[i] = Some(Regex::new(re_str).unwrap())
//         }
// 
//         let mut handlers = vec![None; N];
//         for (i, h) in handlers_for_this_method.into_iter().enumerate() {
//             handlers[i] = Some(h)
//         }
// 
//         Self {
//             regex_set: RegexSet::new(regex_strs).unwrap(),
//             routes:    routes.try_into().unwrap(),
//             handlers:  handlers.try_into().ok().unwrap(),
//         }
//     }
// }
// const _: () = {
//     impl<'router, const N: usize> Router<'router, N> for RegexSetRouter1<'router, N> {
//         fn new(methods: [Method; N], routes: [&'static str; N], handlers: [Handler; N]) -> Self {
//             let this = Se;
//         }
//         fn search<'buf>(&'router self, request_line: &'buf str) -> Option<(&'router Handler, Vec<&'buf str>)> {
//             
//         }
//     }
// };
// 

pub struct RegexSetRouter2 {
    regex_set:    RegexSet,
    routes:       Vec<Regex>,
    handle_funcs: Vec<(HandleFunc, bool/* requires params */)>,
}
const _: () = {
    impl Router for RegexSetRouter2 {
        fn new<const N: usize>(handlers: [Handler; N]) -> Self {
            let handlers = handlers.map(|h| {
                let reuiqres_params = (&h.route).contains(':');
                (h, reuiqres_params)
            });
            let routes = {
                let mut routes = Vec::with_capacity(N);

                let param_pattern = Regex::new(":[a-zA-Z][_a-zA-Z0-9]*").unwrap();
                for (Handler { method, route, .. }, _) in &handlers {
                    let method_name = match method {
                        Method::GET => "GET",
                        Method::POST => "POST",
                        Method::PATCH => "PATCH",
                        Method::DELETE => "DELETE",
                    };
                    routes.push(format!("^{method_name} {}$",
                        param_pattern.replace_all(route, "([_a-zA-Z0-9]+)")
                    ))
                }

                routes
            };

            let mut handle_funcs = Vec::with_capacity(N);
            for (h, rp) in handlers {
                handle_funcs.push((h.proc, rp))
            }

            Self {
                regex_set:    RegexSet::new(&routes).unwrap(),
                routes:       TryInto::<[String; N]>::try_into(routes).unwrap().map(|s| Regex::new(&s).unwrap()).to_vec(),
                handle_funcs,
            }
        }

        #[inline] fn search<'buf>(&self, request_line: &'buf str) -> Option<(&HandleFunc, Vec<&'buf str>)> {
            let matched = self.regex_set.matches(request_line)
                .into_iter()
                .last()?;
            let (handle_func, requires_params) = &self.handle_funcs[matched];
            if *requires_params {
                Some((
                    handle_func,
                    self.routes[matched].captures(request_line).unwrap()
                        .iter().map(|c| c.unwrap().as_str()).collect()
                ))
            } else {
                Some((
                    handle_func,
                    vec![]
                ))
            }
        }
    }
};


#[cfg(test)]
mod test {
    use regex::RegexSet;

    #[test]
    fn how_regex_set_works() {
        let r = RegexSet::new([
            "GET /$",
            "GET /api/users/([_a-zA-Z0-9]+)$",
            "POST /api/users$",
        ]).unwrap();

        let assert_match = |request: &'static str, result: Option<usize>| {
            assert_eq!(r.matches(request).into_iter().next(), result, "{request}")
        };

        assert_match("GET /", Some(0));
        assert_match("GET /api", None);
        assert_match("GET /api/users", None);
        assert_match("GET /api/users/1", Some(1));
        assert_match("POST /api/users", Some(2));
    }
}
