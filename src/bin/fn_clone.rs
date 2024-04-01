use std::future::Future;

#[tokio::main]
async fn main() {
    async fn f(req: usize) -> String {
        format!("Got {req}")
    }

    struct F<
        Proc: Fn(usize) -> Fut + Clone + Send + Sync + 'static,
        Fut:  Future<Output = String> + Send + 'static,
    >(Proc);

    let a = F(f);
    println!("[a] {}", (a.0)(1).await);

    let b = F(f.clone());
    println!("[b] {}", (b.0)(2).await);
}