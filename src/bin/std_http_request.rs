use std::net::TcpStream;
use std::io::{Write, Read, Result};

fn main() -> Result<()> {
    let mut c = TcpStream::connect("www.google.com:80")?;

    c.write_all(b"\
        GET / HTTP/1.1\r\n\
        Host: www.google.com\r\n\
        \r\n\
    ")?;

    let mut buf = [u8::default(); 1 << 16];

    let n = c.read(&mut buf)?;

    println!("\
    ===== escape_ascii =====\n\n\
    {}\n\n\
    ========================\n",
        buf[..n].escape_ascii()
    );

    println!("{}", std::str::from_utf8(&buf[..n]).unwrap());

    Ok(())
}
