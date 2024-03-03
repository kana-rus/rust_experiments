use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct User {
    #[serde(deserialize_with = "User::deserialize_id")]
    id:   usize,
    name: &'static str,
}

impl User {
    fn deserialize_id<'de, D: serde::de::Deserializer<'de>>(
        d: D
    ) -> Result<usize, D::Error> {
        let value = <usize as serde::Deserialize>::deserialize(d)?;

        if !(value >= 1) {
            return Err((|value: usize| serde::de::Error::custom(
                ::std::format!("User's id: expected `value >= 1`, but found `{value}`")
            ))(value))
        }
        if !(value <= 10) {
            return Err((|value: usize| serde::de::Error::custom(
                ::std::format!("User's id: expected `value <= 10`, but found `{value}`")
            ))(value))
        }

        Ok(value)
    }
}

fn main() {
    for input in [
        r#"{"id": 0, "name": "user-0"}"#,
        r#"{"id": 1, "name": "user-1"}"#,
        r#"{"id": 10, "name": "user-10"}"#,
        r#"{"id": 42, "name": "user-42"}"#,
    ] {
        match serde_json::from_str::<'_, User>(input) {
            Ok(user) => println!("{user:?}"),
            Err(err) => println!("{err:?}"),
        };
    }
}
