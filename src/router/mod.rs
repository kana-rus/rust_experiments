#![allow(non_snake_case)]
use std::{pin::Pin, future::Future};

mod trie_tree; pub use trie_tree::TrieTreeRouter;
mod regex_set; pub use regex_set::{RegexSetRouter1, RegexSetRouter2};

pub trait Router<'router, const N: usize> {
    fn register(&mut self, methods: [Method; N], routes: [&'static str; N], handlers: [Handler<'router>; N]);
    /// `request_line` は末尾の ` HTTP/1.1` を除いた `{method} {path}` の形を想定
    fn search<'buf>(&'router self, request_line: &'buf str) -> Option<(&'router Handler, Vec<&'buf str>)>;
}

pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
}

pub struct Request<'buf> {
    method: Method,
    path:   &'buf str,
}

pub enum Response {
    Ok(String),
    Err(String),
}

pub type Handler<'buf> = Box<dyn
    Fn(Request<'buf>) -> Pin<
        Box<dyn
            Future<Output=Response>
            + Send
        >
    > + Send + Sync
>;
