use yarte::Template;

#[derive(Template)]
#[template(src = r#"<html>
    <head>
        <meta lang="en-US">
    </head>
    <body>
        <h1>Hello Sample</h1>
        <p>Hello, {{ name }}!</p>
        <ul>
            <li></li>
            <li></li>
            <p></p>
        </ul>
    </body>
</html>"#)]
struct HelloTemplate {
    name: String,
}

fn main() {
    println!("{}", HelloTemplate {
        name: String::from("kanarus")
    }.call().unwrap());
}
