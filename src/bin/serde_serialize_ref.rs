use std::borrow::Cow;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct User<'req> {
    id:   usize,
    name: Cow<'req, str>
}

fn main() {
    let req: String = String::from(r#"{"id": 42, "name": "kanarus"}"#);
    let u: User = ::serde_json::from_str(&req).unwrap();
    println!("{u:?}");
    println!("name is {}", match u.name {Cow::Borrowed(_)=>"borrowed", Cow::Owned(_)=>"owned"});

    let req: &'static str = r#"{"id": 42, "name": "kanarus"}"#;
    let u: User = ::serde_json::from_str(req).unwrap();
    println!("{u:?}");
    println!("name is {}", match u.name {Cow::Borrowed(_)=>"borrowed", Cow::Owned(_)=>"owned"});

    let u = User { id: 24, name: Cow::Borrowed("ohkami") };
    println!("{u:?}");
    println!("name is {}", match u.name {Cow::Borrowed(_)=>"borrowed", Cow::Owned(_)=>"owned"});
    let res = ::serde_json::to_string(&u).unwrap();
    println!("{res}");
}
