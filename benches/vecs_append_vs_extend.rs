#![feature(test)] extern crate test;


#[bench]
fn append(b: &mut test::Bencher) {
    let mut x = Vec::from(test::black_box("Hello, world!".repeat(4)));
    let mut y = Vec::from(test::black_box("Hello, world!".repeat(5)));

    b.iter(|| {
        x.append(&mut y);
        let _ = x.last();
    })
}

#[bench]
fn extend(b: &mut test::Bencher) {
    let mut x = Vec::from(test::black_box("Hello, world!".repeat(4)));
    let     y = Vec::from(test::black_box("Hello, world!".repeat(5)));

    b.iter(|| {
        x.extend_from_slice(&y);
        let _ = x.last();
    })
}
