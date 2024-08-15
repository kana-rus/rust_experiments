trait Trait: Sized {
    fn f(self) -> Result<Self, std::io::Error>;
}
impl<T> Trait for T {
    fn f(self) -> Result<Self, std::io::Error> {
        Ok(self)
    }
}

fn target<T>(result: Result<T, std::io::Error>) -> Result<T, std::io::Error> {
    result//.and_then(Trait::f)
}

fn main() {
    println!("{:?}", target(std::hint::black_box(Ok("Hello"))));
}
