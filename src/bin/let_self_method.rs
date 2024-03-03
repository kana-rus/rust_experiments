struct S {
    id: i32,
}
impl S {
    fn f(self, name: &str) -> String {
        let S { id } = self;
        format!(r#"""{{"id":{id},"name":"{name}"}}"""#)
    }
}

fn main() {
    let s = S { id: 42 };
    let f = S::f;
}
