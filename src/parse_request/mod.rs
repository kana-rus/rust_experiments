#![allow(unused)]
use std::{ops::{Index, RangeBounds}, slice::SliceIndex};

pub mod via_str;
pub mod u8slice_directly;


const TEST_REQUEST: &'static str = /* `\r\n` is written `\r
` */
"POST /search.html?q1=query&q2=42 HTTP/1.1\r
Host: wa3.i-3-i.info\r
Connection: keep-alive\r
Content-Length: 38\r
Cache-Control: max-age=0\r
Origin: http://wa3.i-3-i.info\r
Upgrade-Insecure-Requests: 1\r
User-Agent: unknown\r
Content-Type: application/x-www-form-urlencoded\r
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8\r
Referer: http://wa3.i-3-i.info/index.html\r
Accept-Encoding: gzip, deflate\r
Accept-Language: ja,en-US;q=0.8,en;q=0.6\r
\r
q=test&submitSearch=%E6%A4%9C%E7%B4%A2";


pub struct Request {
    buffer:  Buffer,
    method:  Method,
    path:    BufRange,
    queries: QueryParams,
    headers: Headers,
    body:    Option<BufRange>,
} impl Request {
    #[inline] fn path(&self) -> &str {
        &self.buffer[&self.path]
    }
    #[inline] fn query(&self, key: &str) -> Option<&str> {
        let QueryParams { params, next } = &self.queries;
        for k_v in &params[..*next as usize] {
            let (k, v) = k_v.as_ref().unwrap();
            if &self.buffer[k] == key {
                return Some(&self.buffer[v])
            }
        }
        None
    }
    #[inline] fn header(&self, key: &str) -> Option<&str> {
        let Headers { headers, next } = &self.headers;
        for k_v in &headers[..*next as usize] {
            let (k, v) = k_v.as_ref().unwrap();
            if &self.buffer[k] == key {
                return Some(&self.buffer[v])
            }
        }
        None
    }
    #[inline] fn body(&self) -> Option<&str> {
        Some(&self.buffer[(&self.body).as_ref()?])
    }
}


const REQUEST_BUFFER_SIZE: usize = 1024;
const QUERY_PARAMS_LIMIT : usize = 4;
const HEADERS_LIMIT      : usize = 32;

type BufRange = std::ops::Range<usize>;
struct Buffer(
    [u8; REQUEST_BUFFER_SIZE]
); const _: () = {
    impl Index<BufRange> for Buffer {
        type Output = str;
        fn index(&self, range: BufRange) -> &Self::Output {
            unsafe {std::str::from_utf8_unchecked(
                &self.0[range]
            )}
        }
    }
    impl<'r> Index<&'r BufRange> for Buffer {
        type Output = str;
        fn index(&self, range: &'r BufRange) -> &Self::Output {
            unsafe {std::str::from_utf8_unchecked(
                &self.0[range.start..range.end]
            )}
        }
    }
};


enum Method {
    GET, POST, PATCH, DELETE,
} impl Method {
    #[inline] fn parse_str(string: &str) -> Self {
        match string {
            "GET" => Self::GET,
            "POST" => Self::POST,
            "PATCH" => Self::PATCH,
            "DELETE" => Self::DELETE,
            _ => panic!("unknown method: `{string}`")
        }
    }
    #[inline] fn parse_bytes(bytes: &[u8]) -> Self {
        match bytes {
            b"GET" => Self::GET,
            b"POST" => Self::POST,
            b"PATCH" => Self::PATCH,
            b"DELETE" => Self::DELETE,
            _ => panic!("unknown method: `{}`", unsafe {std::str::from_utf8_unchecked(bytes)})
        }
    }
}

struct QueryParams {
    params: [Option<(BufRange, BufRange)>; QUERY_PARAMS_LIMIT],
    next:   u8,
} impl QueryParams {
    #[inline] fn new() -> Self {
        Self {
            params: [None, None, None, None],
            next:   0,
        }
    }
    #[inline] fn push(&mut self, key: BufRange, value: BufRange) {
        if self.next == QUERY_PARAMS_LIMIT as u8 {
            panic!("can't handle more than {QUERY_PARAMS_LIMIT} query parameters")
        } else {
            self.params[self.next as usize].replace((key, value));
            self.next += 1
        }
    }
}

struct Headers {
    headers: [Option<(BufRange, BufRange)>; HEADERS_LIMIT],
    next:    u8,
} impl Headers {
    #[inline] fn new() -> Self {
        Self {
            next:    0,
            headers: [
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None,
            ],
        }
    }
    #[inline] fn append(&mut self, key: BufRange, value: BufRange) {
        if self.next == HEADERS_LIMIT as u8 {
            panic!("can't handle more than {HEADERS_LIMIT} request headers")
        } else {
            self.headers[self.next as usize].replace((key, value));
            self.next += 1
        }
    }
}

