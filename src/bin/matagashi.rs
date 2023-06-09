use std::marker::PhantomData;

struct A;
struct B<'b>(PhantomData<fn() -> &'b ()>);
struct C<'c>(PhantomData<fn() -> &'c ()>);

impl A {
    fn b<'a>(&'a self) -> B<'a> {B(PhantomData)}
}
impl<'c> B<'c> {
    fn c(&'c self) -> C<'c> {C(PhantomData)}
}

fn __<'c, 'a:'c>(a: &'a A) -> C<'c> {
    // a.b().c()
    todo!()
}

fn main() {}
