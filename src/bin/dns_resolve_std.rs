use std::net::ToSocketAddrs;

fn main() {
    println!("{:?}", ToSocketAddrs::to_socket_addrs("www.google.com:80"));
}
