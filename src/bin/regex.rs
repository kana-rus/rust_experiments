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
        "^/api(?:$()|/users/([^/]+)$())"
    ).unwrap();
    for path in [
        "/api",
        // "/api/users",
        "/api/users/42",
        "/api/users/kanarus",
    ] {
        println!("\n[{path}]");
        for c in r.captures(path).unwrap().iter() {
            println!("{:?}", c);
        }
    }
}
