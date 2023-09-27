use regex::Regex;

fn main() {
    // let r = Regex::new("^/api$(?<p0>)").unwrap();
    // let captures = r.captures("/api").unwrap();
    // for c in captures.iter() {
    //     println!("[c] {:?}", c.unwrap())
    // }
// 
    // println!();
// 
    // let r = Regex::new("^/api/users/([^/]+)$(?<p1>)").unwrap();
    // let captures = r.captures("/api/users/42").unwrap();
    // for c in captures.iter() {
    //     println!("[c] {:?}", c.unwrap());
    // }
// 
    // println!();
// 
    let r = Regex::new(
        // "^/api(?:$()|/users/([^/]+)$())"
        // "^/api(?:$|/users$)"
        "^/api(?:$()|/(?:users/([^/]+)$()|tasks/([^/]+)$()))"
    ).unwrap();
    for path in [
        "/api",
        "/api/users/42",
        "/api/users/kanarus",
        "/api/tasks/42",
        "/api/tasks/kanarus",
    ] {
        println!("\n[{path}]");
        for c in r.captures(path).unwrap().iter().skip(1/*The entire match*/) {
            println!("{:?}", c);
        }
    }
}
