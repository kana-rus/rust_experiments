use std::{pin::Pin, future::Future};

type F<'arg> = Box<dyn
    Fn(&'arg str) -> Pin<Box<dyn
        Future<Output = ()>
        + Send
    >>
+ Sync + Send>;

trait IntoF<'arg> {fn into_f(self) -> F<'arg>;}
impl<'arg, Func, Fut> IntoF<'arg> for Func
where
    Func: Fn(&'arg str) -> Fut + Sync + Send + 'static,
    Fut:  Future<Output = ()> + Send + 'static,
{
    fn into_f(self) -> F<'arg> {
        Box::new(move |arg| Box::pin(self(arg)))
    }
}

struct Store<'arg>(
    Vec<F<'arg>>
); impl<'arg> Store<'arg> {
    fn new() -> Self {
        Self(vec![])
    }
    fn add<Func: IntoF<'arg>>(&mut self, f: Func) {
        self.0.push(f.into_f())
    }
    fn get(&self, index: usize) -> Option<&F<'arg>> {
        self.0.get(index)
    }
}

async fn a(name: &str) {println!("Hello, {name}!")}


fn _main() {
    let store: &'static mut Store = Box::leak(
        Box::new(
            Store::new()
        )
    );
    store.add(a);

    let proc = store.get(0).unwrap();
    async_std::task::spawn(async {
        proc("world").await
    });
}

fn main() {
    let mut store = Store::new();
    store.add(a);

    let proc = store.get(0).unwrap();
    async_std::task::block_on(async {
        proc("world").await;
    });
}
