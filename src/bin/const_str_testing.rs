#![allow(
    incomplete_features,
    unused,
)]
#![feature(
    generic_const_exprs,
    adt_const_params,
    // specialization,
)]

// struct ConstType<const C: &'static str>;
// 
// trait IsEmpty {
//     const YES: bool;
// }
// impl<const C: &'static str> IsEmpty for ConstType<C> {
//     default const YES: bool = false;
// }
// impl IsEmpty for ConstType<""> {
//     const YES: bool = true;
// }
// 
//         
// const C1: bool = <ConstType<""> as IsEmpty>::YES;
// const C2: bool = <ConstType<"rust"> as IsEmpty>::YES;
// 
// 
// fn main() {
//     // let c = ConstType::<"">::new();
// }

// ====

// struct Const<const S: &'static str>;
// struct Union<const VARIANTS: &'static [&'static str]>;

const fn is_one_of(s: &'static str, variants: &'static [&'static str]) -> bool {
    const fn eq(s1: &'static str, s2: &'static str) -> bool {
        let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
        if s1.len() != s2.len() {return false}

        let mut i = 0;
        while i < s1.len() {
            if s1[i] != s2[i] {return false}
            i += 1
        }
        true
    }

    let mut i = 0;
    while i < variants.len() {
        if eq(s, variants[i]) {return true}
        i += 1
    }
    false
}

// trait IsOneOf<const VARIANTS: &'static [&'static str], const YES: bool> {}
// trait OneOf<const VARIANTS: &'static [&'static str]> {}
// const _: () = {
//     impl<const S: &'static str, const VARIANTS: &'static [&'static str]>
//     IsOneOf<VARIANTS, {is_one_of(S, VARIANTS)}> for Const<S> {} 
// 
//     impl<const S: &'static str, const VARIANTS: &'static [&'static str]>
//     OneOf<VARIANTS> for Const<S>
//     where
//         Self: IsOneOf<VARIANTS, true>
//     {}
// };

// impl<const S: &'static str, const VARIANTS: &'static [&'static str]> Const<S>
// where
//     Self: OneOf<VARIANTS>
// {
//     fn is_one_of(&self, union: Union<VARIANTS>) {}
// }


struct Lang<const NAME: &'static str>;
const MY_LANGS: &[&str] = &[
    "Rust",
    "Go",
    "TypeScript",
    "C",
];

const _: (/* 黒魔術 */) = {
    trait IsMyLang<const YES: bool> {}
    trait MyLang {}
    impl<const NAME: &'static str> IsMyLang<{is_one_of(NAME, MY_LANGS)}> for Lang<NAME> {}
    impl<const NAME: &'static str> MyLang for Lang<NAME> where Self: IsMyLang<true> {}

    impl<const NAME: &'static str> Lang<NAME>
    where
        Self: MyLang
    {
        fn is_my_lang(&self) {}
    }
};

fn main() {
    let rust = Lang::<"Rust">;
    rust.is_my_lang();

    let go = Lang::<"Go">;
    go.is_my_lang();

    let kotlin = Lang::<"Kotlin">;
    kotlin.is_my_lang();

    let c = Lang::<"C">;
    c.is_my_lang();
}
