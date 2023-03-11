#![feature(test)]
extern crate test;
use test::Bencher;

use experiment::parse_request::{Request, REQUEST_BUFFER_SIZE, TEST_REQUEST};


#[bench]
fn parse_via_str(b: &mut Bencher) {
    let test_case: [u8; REQUEST_BUFFER_SIZE] = {
        let mut buffer = TEST_REQUEST.as_bytes().to_vec();
        buffer.resize(REQUEST_BUFFER_SIZE, 0/*null*/);
        buffer.try_into().unwrap()
    };
    b.iter(|| for _ in 0..100 {
        let request = Request::parse_via_str(test_case);
        println!("{}", request.path());
    })
}

#[bench]
fn parse_u8slice_directly(b: &mut Bencher) {
    let test_case: [u8; REQUEST_BUFFER_SIZE] = {
        let mut buffer = TEST_REQUEST.as_bytes().to_vec();
        buffer.resize(REQUEST_BUFFER_SIZE, 0/*null*/);
        buffer.try_into().unwrap()
    };
    b.iter(|| for _ in 0..100 {
        let request = Request::parse_directly(test_case);
        println!("{}", request.path());
    })
}
