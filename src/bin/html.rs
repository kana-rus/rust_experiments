macro_rules! html {
    (<$tag:ident>) => {
        {
            let _: Tag = Tag::$tag;
            concat!("<", stringify!($tag), ">")
        }
    };
}

#[allow(non_camel_case_types)]
enum Tag {
    h1,
    h2,
    p,
    a,
    span,
    div,
}


fn main() {
    let h1 = html!(<h2>);
    println!("{h1}")
}
