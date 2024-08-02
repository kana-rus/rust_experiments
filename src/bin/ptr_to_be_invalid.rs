fn main() {
    let s = String::from("Hello, world!");

    let p = &s as *const String;

    dbg!(p, p.is_null(), p.is_aligned(), unsafe {p.as_ref()});

    println!("{s}");

    dbg!(p, p.is_null(), p.is_aligned(), unsafe {p.as_ref()});

    drop(s);

    dbg!(p, p.is_null(), p.is_aligned(), unsafe {p.as_ref()});
}
