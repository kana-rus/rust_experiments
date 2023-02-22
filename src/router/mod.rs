#![allow(non_snake_case)]
use std::{pin::Pin, future::Future};

mod trie_tree; pub use trie_tree::TrieTreeRouter;
mod regex_set; pub use regex_set::RegexSetRouter;

pub trait Router<'router> {
    fn register(&mut self, method: Method, route: &'static str, handler: Handler<'router>);
    fn search<'buf>(&'router self, method: Method, path: &'buf str) -> Option<(&'router Handler, Vec<&'buf str>)>;
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
