use serde::Serialize;

#[derive(Debug)]
enum MyError {}
impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

struct PetListTemplate {
    pets: Vec<Pet>,
}
struct Pet {
    name: String,
    age:  Option<usize>,
}

impl PetListTemplate {
    fn render(&self) -> Result<String, MyError> {
        Ok(format!("\
            <!DOCTYPE html>
            <html>
            <head>
                <meta lang=\"en-US\" charset=\"UTF-8\">
            </head>
            <body>
                <ul>{}</ul>
            </body>
            </html>
        ",
            self.pets.iter()
                .map(|p| format!("\
                    <li>{} ({})</li>
                ",
                    p.name,
                    p.age.map(|a| a.to_string()).unwrap_or_else(|| "?".to_string()),
                )).collect::<Vec<_>>()
                .join("")
        ))
    }
}

impl Serialize for PetListTemplate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        let template = self.render().map_err(|e| serde::ser::Error::custom(e.to_string()))?;
        serializer.serialize_str(&template)
    }
}


fn main() {

}
