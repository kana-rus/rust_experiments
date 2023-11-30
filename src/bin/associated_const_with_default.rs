trait T {
    const C: u8 = 42;
}

impl T for () {}






fn main() {
    println!("{}", <()>::C);
}
