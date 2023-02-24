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
//     handlers:  [Option<Handler<'router>>; N],
// } impl<'router, const N: usize> Node<'router, N> {
//     fn new(regex_strs: Vec<String>, handlers_for_this_method: Vec<Handler<'router>>) -> Self {
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
//         fn new(methods: [Method; N], routes: [&'static str; N], handlers: [Handler<'router>; N]) -> Self {
//             let this = Se;
//         }
//         fn search<'buf>(&'router self, request_line: &'buf str) -> Option<(&'router Handler, Vec<&'buf str>)> {
//             
//         }
//     }
// };
// 

pub struct RegexSetRouter2<'router, const N: usize> {
    regex_set:    RegexSet,
    routes:       [Regex; N],
    handle_funcs: [HandleFunc<'router>; N],
}
const _: () = {
    impl<'router, const N: usize> Router<'router, N> for RegexSetRouter2<'router, N> {
        fn new(handlers: [Handler<'router>; N]) -> Self {
            let routes = {
                let mut routes = Vec::with_capacity(N);

                let param_pattern = Regex::new(":[a-zA-Z][_a-zA-Z0-9]*/").unwrap();
                for Handler { method, route, .. } in &handlers {
                    let method_name = match method {
                        Method::GET => "GET",
                        Method::POST => "POST",
                        Method::PATCH => "PATCH",
                        Method::DELETE => "DELETE",
                    };
                    routes.push(format!("{method_name} {}",
                        param_pattern.replace_all(route, "([_a-zA-Z0-9]+)/")
                    ))
                }

                routes
            };

            Self {
                regex_set:    RegexSet::new(&routes).unwrap(),
                routes:       TryInto::<[String; N]>::try_into(routes).unwrap().map(|s| Regex::new(&s).unwrap()),
                handle_funcs: handlers.map(|h| h.proc),
            }
        }

        fn search<'buf>(&'router self, request_line: &'buf str) -> Option<(&'router HandleFunc, Vec<&'buf str>)> {
            let matched = self.regex_set.matches(request_line)
                .into_iter()
                .last()?;
            Some((
                &self.handle_funcs[matched],
                self.routes[matched].captures(request_line).unwrap()
                    .iter().map(|c| c.unwrap().as_str()).collect()
            ))
        }
    }
};
