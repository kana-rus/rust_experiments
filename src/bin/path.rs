use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from("~/.config");
    assert!(path.exists());

    path.push("../.cargo");
    assert!(path.exists());
}