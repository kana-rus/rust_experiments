use std::{pin::Pin, future::Future, time::Duration};

use tokio::time::sleep;


async fn fang_1(mut c: String) -> String {
    sleep(Duration::from_secs(1)).await;
    c.push('1');
    println!("c: \"{c}\" in fang_1");
    c
}

async fn fang_2(mut c: String) -> String {
    sleep(Duration::from_secs(2)).await;
    c.push('2');
    println!("c: \"{c}\" in fang_2");
    c
}

async fn fang_3(mut c: String) -> String {
    sleep(Duration::from_secs(3)).await;
    c.push('3');
    println!("c: \"{c}\" in fang_3");
    c
}

async fn fang_4(mut c: String) -> String {
    sleep(Duration::from_secs(4)).await;
    c.push('4');
    println!("c: \"{c}\" in fang_4");
    c
}

async fn handler(mut c: String) -> String {
    c.push_str("handler");
    c
}

#[tokio::main]
async fn main() {
    let mut c = String::new();

    let fangs: Vec<Box<dyn
        Fn(String) -> Pin<Box<dyn Future<Output = String>>>
    >> = vec![
        Box::new(|c| Box::pin(fang_1(c))),
        Box::new(|c| Box::pin(fang_2(c))),
        Box::new(|c| Box::pin(fang_3(c))),
        Box::new(|c| Box::pin(fang_4(c))),
    ];

    for f in &fangs {
        c = f(c).await
    }
    c = handler(c).await;

    println!("{c}")
}
