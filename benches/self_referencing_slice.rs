#![feature(test)]
extern crate test;
use std::{mem::MaybeUninit, ops::Range};
use byte_reader::Reader;


const HEADER_LIMIT: usize = 16;
const HEADERS: &[(&str, &str)] = &[
    ("User-Agent",      "Mozilla/4.0 (compatible; MSIE5.01; Windows NT)"),
    ("Host",            "www.tutorialspoint.com"),
    ("Content-Type",    "application/x-www-form-urlencoded"),
    ("Content-Length",  "length"),
    ("Accept-Language", "en-us"),
    ("Accept-Encoding", "gzip, deflate"),
    ("Connection",      "Keep-Alive"),
];
const DUMMY_HEADERS: &[&str] = &[
    "X-App-Token",
    "X-API-Key",
    "Accept",
    "Referer",
    "Cache-Control",
];
fn headers() -> String {
    HEADERS.iter()
        .map(|(k, v)| format!("{k}: {v}\r\n"))
        .fold(String::new(), |mut s, h| {s.push_str(&h); s})
        + "\r\n"
}

struct List<T> {
    next:     usize,
    elements: [MaybeUninit<T>; HEADER_LIMIT],
} impl<T> List<T> {
    fn new() -> Self {
        Self { next: 0, elements: [
            MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(),
            MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(),
            MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(),
            MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(), MaybeUninit::uninit(),
        ]}
    }
    fn push(&mut self, element: T) {
        assert!(self.next < HEADER_LIMIT);
        self.elements[self.next].write(element);
        self.next += 1;
    }
    fn iter(&self) -> impl Iterator<Item = &T> {
        (&self.elements[..self.next])
            .into_iter()
            .map(|mu| unsafe {mu.assume_init_ref()})
    }
}


struct RequestWithRange {
    buf:     [u8; 1024],
    headers: List<(Range<usize>, Range<usize>)>,
} impl RequestWithRange {
    fn new(input: String) -> Self {
        let mut buf = [0; 1024];
        for (i, b) in input.into_bytes().into_iter().enumerate() {buf[i] = b}

        let mut headers = List::new();

        let mut r = Reader::from(&buf);
        let mut start = 0; while r.peek().unwrap() != &b'\r' {
            let key_len = r.read_while(|b| b != &b':').len();
            r.consume(": ").unwrap();
            let value_len = r.read_while(|b| b != &b'\r').len();
            r.consume("\r\n").unwrap();

            headers.push((start..(start+key_len), (start+key_len+2)..(start+key_len+2+value_len)));
            start += key_len + 2 + value_len + 2;
        }

        Self { buf, headers }
    }
    fn header(&self, key: &str) -> Option<&str> {
        let key_bytes = key.as_bytes();
        for (k, v) in self.headers.iter() {
            if key_bytes.eq_ignore_ascii_case(&self.buf[k.start..k.end]) {
                return std::str::from_utf8(&self.buf[v.start..v.end]).ok()
            }
        }
        None
    }
}

struct RequestWithPtrAndLen {
    headers: List<((*const u8, usize), (*const u8, usize))>,
} impl RequestWithPtrAndLen {
    fn new(input: String) -> Self {
        let mut buf = [0; 1024];
        for (i, b) in input.into_bytes().into_iter().enumerate() {buf[i] = b}

        let mut headers = List::new();

        let mut r = Reader::from(&buf);
        while r.peek().unwrap() != &b'\r' {
            let key = r.read_while(|b| b != &b':');
            let (kp, klen) = (key.as_ptr(), key.len());
            r.consume(": ").unwrap();
            let value = r.read_while(|b| b != &b'\r');
            let (vp, vlen) = (value.as_ptr(), value.len());
            r.consume("\r\n").unwrap();

            headers.push(((kp, klen), (vp, vlen)))
        }

        Self { headers }
    }
    fn header(&self, key: &str) -> Option<&str> {
        let key_bytes = key.as_bytes();
        for ((kp, klen), (vp, vlen)) in self.headers.iter() {
            if key_bytes.eq_ignore_ascii_case(unsafe {std::slice::from_raw_parts(*kp, *klen)}) {
                return std::str::from_utf8(unsafe {std::slice::from_raw_parts(*vp, *vlen)}).ok()
            }
        }
        None
    }
}


macro_rules! benchmark {
    ($($target:ident)*) => {$(
        #[bench] #[allow(non_snake_case)] fn $target(b: &mut test::Bencher) {
            let req = $target::new(headers());
            b.iter(|| for _ in 0..10 {
                for (k, v) in HEADERS {
                    assert_eq!(req.header(k), Some(*v))
                }
                for d in DUMMY_HEADERS {
                    assert!(req.header(d).is_none());
                }
            })
        }        
    )*};
} benchmark! {
    RequestWithRange
    RequestWithPtrAndLen
}
