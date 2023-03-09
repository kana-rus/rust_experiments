use std::{pin::Pin, future::Future};

type F = Box<dyn
    for <'arg> Fn(&'arg str) -> Pin<Box<dyn
        Future<Output = String>
    >> + Send
+ Sync + Send>;

trait IntoF {fn into_f(self) -> F;}
impl<Func, Fut> IntoF for Func
where
    Func: for <'arg> Fn(&'arg str) -> Fut + Sync + Send + 'static,
    Fut:  Future<Output = String> + 'static,
{
    fn into_f(self) -> F {
        Box::new(move |arg| Box::pin(self(arg)))
    }
}

struct Store(
    Vec<F>
); impl Store {
    fn new() -> Self {
        Self(vec![])
    }
    fn add<Func: IntoF>(&mut self, f: Func) {
        self.0.push(f.into_f())
    }
    fn get(&self, index: usize) -> Option<&F> {
        self.0.get(index)
    }
}

async fn a(name: &str) -> String {format!("Hello, {name}!")}

fn main() {
    let mut store = Store::new();
    // store.add(a);  // <--

    // let proc = store.get(0).unwrap();
    // async_std::task::block_on(async {
    //     let message = proc("world").await;
    //     println!("{message}")
    // });
}
