use std::future::Future;

struct P<Arg, Fut: Future<Output = String>>(
    fn(Arg)->Fut
);

async fn hello(to: &str) -> String {
    format!("Hello, {to}!")
}

#[tokio::main]
async fn main() {
    let p = P(hello);

    println!("{}", (p.0)("ohkami").await)
}
