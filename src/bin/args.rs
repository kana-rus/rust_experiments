#[derive(Debug)]
struct Config {
    query:    String,
    filename: String,
}

impl Config {
    fn from_cli_args() -> Result<Self, &'static str> {
        let mut args = std::env::args().skip(1);

        if args.len() < 2 {
            return Err("Not enough arguments");
        }

        Ok(Self {
            query:    args.next().unwrap(),
            filename: args.next().unwrap(),
        })
    }
}

fn main() -> Result<(), &'static str> {
    let config = Config::from_cli_args()?;

    println!("config: {config:?}");

    Ok(())
}
