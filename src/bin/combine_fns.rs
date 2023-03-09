#![feature(unboxed_closures, fn_traits)]
use std::{pin::Pin, future::Future};

struct Arg<'arg>(&'arg str);
struct Data<'data>(&'data str);


struct F<'arg>(
    Box<dyn
        Fn(Arg<'arg>, &'arg Data<'arg>) -> Pin<
            Box<dyn
                Future<Output = Arg<'arg>>
                + Send + 'arg
            >
        > + Send + Sync + 'arg
    >
); impl<'arg> F<'arg> {
    fn clone<'this: 'arg>(&'this self) -> Self {
        Self(Box::new(|arg, data| Box::pin(
                (self.0)(arg, data)
        )))
    }
}

trait IntoF<'arg> {
    fn into_f(self) -> F<'arg>;
}
impl<'arg, Func, Fut> IntoF<'arg> for Func
where
    Func: Fn(Arg<'arg>, &'arg Data<'arg>) -> Fut + Send + Sync + 'arg,
    Fut:  Future<Output = Arg<'arg>> + Send + 'arg
{
    fn into_f(self) -> F<'arg> {
        F(Box::new(move |c, d| Box::pin(
            self(c, d)
        )))
    }
}

struct Store<'arg>(Vec<F<'arg>>);
impl<'arg> Store<'arg> {
    fn new() -> Self {
        Self(Vec::new())
    }
    fn push<Func: IntoF<'arg>>(&mut self, func: Func) {
        self.0.push(func.into_f())
    }
}

/*
    lifetime may not live long enough
    returning this value requires that `'arg` must outlive `'static`
*/
fn combine<'arg>(this: &'arg F<'arg>, another: &'arg F<'arg>) -> F<'arg> {
    F(Box::new(move |mut arg, data| Box::pin(async move {
        arg = this.0(arg, data).await;
        arg = another.0(arg, data).await;
        arg
    })))
}
fn combine_owned<'arg>(this: F<'arg>, another: F<'arg>) -> F<'arg> {
    F(Box::new({
        move |mut arg, data| {
            let this = this.clone();
            let another = another.clone();
            Box::pin(async move {
                arg = (this.0)(arg, data).await;
                arg = (another.0)(arg, data).await;
                arg
            })
        }
    }))
}

fn main() {
    let mut store = Store::new();
    async fn a<'arg>(a_arg: Arg<'arg>, _: &'arg Data<'arg>) -> Arg<'arg> {a_arg}
    async fn b<'arg>(b_arg: Arg<'arg>, _: &'arg Data<'arg>) -> Arg<'arg> {b_arg}

    store.push(a);
}
