use std::{marker::PhantomData, mem::size_of};


struct S1<'a>(PhantomData<&'a()>);
struct S2<'a>(#[allow(unused)] &'a ());
type S3 = ();

fn main() {
    println!("S1's size: {}", size_of::<S1>());
    println!("S2's size: {}", size_of::<S2>());

    #[derive(serde::Serialize)]
    struct J1<'a> {
        v: PhantomData<&'a()>
    }
    #[derive(serde::Serialize)]
    struct J2<'a> {
        v: &'a()
    }
    #[derive(serde::Serialize)]
    struct J3 {
        v: PhantomData<serde_json::Value>
    }

    println!("{}", serde_json::to_string(&J1 {
        v: PhantomData
    }).unwrap());
    println!("{}", serde_json::to_string(&J2 {
        v: &()
    }).unwrap());
    println!("{}", serde_json::to_string(&J3 {
        v: PhantomData
    }).unwrap());
}



/*

`PhantomData<&'a ()>` なら zero-sized なのに対して、
unit 自体は zero-sized ( https://doc.rust-lang.org/reference/type-layout.html#tuple-layout ) ですが `&'a ()` は 1 word ( https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=aa08f71085c2a767e393f22f55432ba4 ) ですし、コンパイラとしても勝手に消して挙動が変わらない保証がない ( わかりやすい例として、`serde_json` において unit は JSON の null に対応します ) ので「コンパイル時に削除される」こともないはずです。
個人的には素直に `PhantomData` を使うのがいいように思います。

*/