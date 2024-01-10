enum MyError {}

fn main() {
    fn assert_impl<T: serde::Serialize>() {}

    assert_impl::<String>();
    assert_impl::<Result<String, String>>();
    // assert_impl::<Result<String, MyError>>();
}
