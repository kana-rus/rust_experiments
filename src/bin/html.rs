macro_rules! html {
    (<$tag_open:ident></$tag_close:ident>) => {
        {
            let _: Tag = Tag::$tag_open;
            let _: Tag = Tag::$tag_close;
            concat!("<", stringify!($tag), ">")
            "<h1></h1>"
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
    // let h1 = html!(<h);
    // println!("{h1}")
}
