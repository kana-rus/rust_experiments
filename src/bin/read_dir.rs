use std::{env, fs};

fn main() {
    let cd = dbg!(env::current_dir().unwrap());
    let files = fs::read_dir("..").unwrap()
        .map(|r| r.unwrap().file_name().into_string().unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{files}")
}