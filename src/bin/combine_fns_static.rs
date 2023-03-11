use std::{pin::Pin, future::Future};

struct Arg;
#[derive(Clone)]
struct Data;


struct F(
    Box<dyn
        Fn(Arg, Data) -> Pin<
            Box<dyn
                Future<Output = (Arg, Data)>
                + Send
            >
        > + Send + Sync
    >
); impl F {
    fn clone(&'static self) -> Self {
        F(Box::new(|arg, data| {
            Box::pin({
                self.0(arg, data)
            })
        }))
    }
}

trait IntoF<E> {
    fn into_f(&'static self) -> F;
}
/*
    `data_cloned` does not live long enough
    borrowed value does not live long enough
*/
// impl<'d, Func, Fut> IntoF<(Fut)> for Func
// where
//     Func: Fn(Arg, &'d Data) -> Fut + Send + Sync + 'static,
//     Fut:  Future<Output = Arg> + Send + 'static
// {
//     fn into_f(&'static self) -> F {
//         F(Box::new(|c, d| Box::pin({
//             let data_cloned = d.clone();
//             async {
//                 let arg = self(c, &data_cloned).await;
//                 (arg, d)
//             }
//         })))
//     }
// }

struct Store(Vec<F>);
impl Store {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push<E, Func: IntoF<E> + 'static>(&mut self, func: &'static Func) {
        self.0.push(func.into_f())
    }
}

/*
    lifetime may not live long enough
    returning this value requires that `'arg` must outlive `'static`
*/
fn combine(this: &'static F, another: &'static F) -> F {
    F(Box::new(|mut arg, mut data| Box::pin({
        let this = this.clone();
        let another = another.clone();
        async move {
            (arg, data) = this.0(arg, data).await;
            another.0(arg, data).await
        }
    })))
}

fn main() {
    let mut store = Store::new();
    async fn a(a_arg: Arg, _: &Data) -> Arg {a_arg}
    async fn b(b_arg: Arg, _: &Data) -> Arg {b_arg}

    // store.push(&a);
}
