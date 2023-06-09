#![feature(unboxed_closures, fn_traits)]

#![allow(incomplete_features)]
#![feature(adt_const_params)]

#![allow(non_snake_case)]


pub trait Model {
    const TABLE_NAME: &'static str;
}

pub struct Condition(String);
pub struct NumberCondition<const NAME: &'static str>;
impl<const NAME: &'static str> NumberCondition<NAME> {
    pub fn eq<N: condition_builder::Number>(self, number: N) -> Condition {
        Condition(format!("{NAME} = {number}"))
    }
}
pub struct StringCondition<const NAME: &'static str>;
impl<const NAME: &'static str> StringCondition<NAME> {
    pub fn eq<S: condition_builder::Str>(self, string: S) -> Condition {
        Condition(format!("{NAME} = '{string}'"))
    }
}
mod condition_builder {
    pub trait Number: std::fmt::Display {}
    impl Number for usize {}
    impl Number for u8 {}
    // ...

    pub trait Str: std::fmt::Display {}
    impl Str for String {}
    impl Str for &str {}
}


pub struct User {
    pub id:       usize,
    pub name:     String,
    pub password: String,
} impl User {
    pub fn WHERE<F: Fn(__::UserCondition)->Condition>(condition: F) -> __::UserPredicate {
        __::UserPredicate(condition(__::UserCondition::new()))
    }
} mod __ {
    use super::*;

    pub struct UserCondition {
        pub id:       NumberCondition<"id">,
        pub name:     StringCondition<"name">,
        pub password: StringCondition<"password">,
    }
    impl UserCondition {
        #[inline] pub(super) fn new() -> Self {
            Self {
                id: NumberCondition,
                name: StringCondition,
                password: StringCondition,
            }
        }
    }

    pub struct UserPredicate(pub(super) Condition);
    impl UserPredicate {
        pub async fn First(&self) -> User {
            User {
                id: 1,
                name: String::from("sample_user"),
                password: String::from("password"),
            }
        }
    }

    impl Model for User {
        const TABLE_NAME: &'static str = "users";
    }
}




#[tokio::main]
async fn main() {
    // let user = User();
}
