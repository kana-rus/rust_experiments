// #![feature(unboxed_closures, fn_traits)]

trait Winnow<I, P> {
    fn winnow(self, pred: P) -> WinnowIterator<I, P>;}

struct WinnowIterator<I, P>(I, P);

struct WinnowType;


// impl<'a, I:Iterator<Item=i32>, P: Fn(&'a i32)->bool>
// Winnow<I, P> for I {
//     fn winnow(self, pred: P) -> WinnowIterator<I, P> {WinnowIterator(self, pred)}}
impl<I: Iterator<Item=i32>, P: Fn(&i32)->bool>
Winnow<I, P> for I {
    fn winnow(self, _: P) -> WinnowIterator<I, P> {todo!()}}

impl<I: Iterator<Item=i32>>
Winnow<I, WinnowType> for I {
    fn winnow(self, _: WinnowType) -> WinnowIterator<I, WinnowType> {todo!()}}


fn main() {
    fn f(i: &i32) -> bool {i & 1 == 0}

    (0..10).filter(|i| i & 1 == 0);
    (0..10).filter(f);
    (0..10).winnow(|i: &_| i & 1 == 0);
    (0..10).winnow(f);
}




#[cfg(hoge)] const _: () = {
    impl FnOnce<(&i32,)> for WinnowType {
        type Output = bool;
        extern "rust-call" fn call_once(self, _: (&i32,)) -> Self::Output {todo!()}}
    impl FnMut<(&i32,)> for WinnowType {
        extern "rust-call" fn call_mut(&mut self, _: (&i32,)) -> Self::Output {todo!()}}

    // impl Fn<(&i32,)> for WinnowType {
    //     extern "rust-call" fn call(&self, _: (&i32,)) -> Self::Output {todo!()}}
};
