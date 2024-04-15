use ohkami::prelude::*;
use ohkami::{typed::Payload, builtin::payload::JSON};

#[derive(Clone)]
struct Logger;
impl FangAction for Logger {
    fn fore<'a>(&'a self, req: &'a mut Request) -> impl std::future::Future<Output = Result<(), Response>> + Send {
        tracing::info!("\n{req:?}");
        async {Ok(())}
    }
    fn back<'a>(&'a self, res: &'a mut Response) -> impl std::future::Future<Output = ()> + Send {
        tracing::info!("\n{res:?}");
        async {}
    }
}

#[Payload(JSON/S)]
struct Message {
    message: String
}

async fn hello(name: &str) -> Message {
    Message {
        message: format!("Hello, {name}!")
    }
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(tracing::Level::INFO).init();

    Ohkami::with(Logger, (
        "/hello/:name".GET(hello),
    )).howl("localhost:3000").await
}
