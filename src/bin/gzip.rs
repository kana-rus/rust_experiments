use std::io::{Write, Read};
use libflate::gzip;

const INVITE_CODE: &str = "bsky-social-wxcbq-weelp";

fn main() {
    let mut encoder = gzip::Encoder::new(Vec::new()).unwrap();
    encoder.write_all(INVITE_CODE.as_bytes()).unwrap();
    let encoded = encoder.finish().into_result().unwrap();
    println!("[encoded]\n{encoded:?} (length: {})\n", encoded.len());
    println!("[escaped]\n{}\n", encoded.escape_ascii());

    let mut decoder = gzip::Decoder::new(&encoded[..]).unwrap();
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    assert_eq!(decoded, INVITE_CODE.as_bytes());
    println!("[decoded]\n{decoded:?} (length: {})\n", decoded.len());
    println!("[escaped]\n{}\n", decoded.escape_ascii());

    println!("\
        ---\r\n\
        HTTP/1.1 200 OK\r\n\
        Content-Encoding: gzip\r\n\
        Content-Length: 50\r\n\
        \r\n\
        {}\r\n\
        ---\r\n\
    ", encoded.escape_ascii());

    println!();
    let x = b"\x1f\x8b\x08\x00\xcc\x9b\xccd\x00\x03\x05\xc0\xc1\r\x00@\x04\x04\xc0\x8a\xb6\xa8#\x1e\x17\x12\xc4\x03\xdd\x1b*]\x94\xf3\x7f\x86\x1e\xa6D\x8bX\x1c\xc1\xa4\xcbD\x17\x00\x00\x00";
    let mut decoder = gzip::Decoder::new(&x[..]).unwrap();
    let mut decoded = Vec::new();
    decoder.read_to_end(&mut decoded).unwrap();
    assert_eq!(decoded, INVITE_CODE.as_bytes());
    println!("[decoded]\n{decoded:?} (length: {})\n", decoded.len());
    println!("[escaped]\n{}\n", decoded.escape_ascii());
}
