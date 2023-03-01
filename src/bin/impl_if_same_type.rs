#![feature(const_type_name)]
#![allow(unused)]

use std::ops::{Add, Sub, AddAssign};
use std::any::type_name;

struct S(usize);
struct T(usize);
struct Out(usize);

type Alias = T;

impl Add<T> for S {
    type Output = Out;
    fn add(self, rhs: T) -> Self::Output {
        Out(self.0 + rhs.0)
    }
}









struct A(usize);
struct B(usize);

const fn eq(lhstr: &'static str, rhstr: &'static str) -> bool {
    let lhs = lhstr.as_bytes();
    let rhs = rhstr.as_bytes();

    if lhs.len() != rhs.len() {
        return false;
    }

    let mut i = 0;
    while i < lhs.len() {
        if lhs[i] != rhs[i] {
            return false;
        }
        i += 1;
    }

    true
}

/*
fn sample_1() {
    struct X(usize);

    const _: (/* for input
    ```
        impl Add<B> for A {
            type Output = X;
            fn add(self, rhs: B) -> Self::Output {
                X(self.0 + rhs.0)
            }
        }
    ```
    */) = {
        const fn impl_add_if_same_type() {
            if eq(type_name::<X>(), type_name::<A>()) {
                {
                    impl Add<B> for A {
                        type Output = X;
                        fn add(self, rhs: B) -> Self::Output {
                            X(self.0 + rhs.0)
                        }
                    }
                    impl AddAssign<B> for A {
                        fn add_assign(&mut self, rhs: B) {
                            self = *self + rhs
                        }
                    }
                }
            } else {
                {

                }
            }
        }
        impl_add_if_same_type()
    };
}
*/

/* 
impl Add<B> for A {
    type Output = X;
    fn add(self, rhs: B) -> Self::Output {
        X(self.0 + rhs.0)
    }
}

struct X(usize);

#[cfg(eq())]
impl AddAssign<B> for A {

}
*/

fn main() {}
