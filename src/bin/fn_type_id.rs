use std::{any::{Any, TypeId}, future::Future};


fn id<F: Fn() -> Fut + 'static, Fut: Future<Output = ()>>(f: F) -> TypeId {
    <F as Any>::type_id(&f)
}

async fn f1() {}

async fn f2() {}

fn main() {
    println!("{:?}", id(f1));
    println!("{:?}", id(f2));
}
