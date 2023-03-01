#![feature(const_type_name)]
#![allow(unused)]

use std::ops::{Add, Sub, AddAssign};
use std::any::type_name;

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

#[derive(Debug, PartialEq, Clone)]
struct A(usize);
struct B(usize);

impl Add<B> for A {
    type Output = X;
    fn add(self, rhs: B) -> Self::Output {
        A(self.0 + rhs.0)
    }
}

type X = A;

const _: () = {
    const fn impl_assgin_if_same_type() {
        if eq(type_name::<A>(), type_name::<<A as Add<B>>::Output>()) {
            impl AddAssign<B> for A {
                fn add_assign(&mut self, rhs: B) {
                    *self = self.clone() + rhs
                }
            }
        }
    }
    //impl_assgin_if_same_type()
};

fn main() {
    let x = A(100) + B(200);
    assert_eq!(x, A(300));

    let mut a = A(100);
    a += B(200);
    assert_eq!(a, A(300));
}
