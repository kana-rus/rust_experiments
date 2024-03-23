use ohkami::prelude::*;

struct RequestLogger;
impl FrontFang for RequestLogger {
    type Error = std::convert::Infallible;
    fn bite(&self, req: &mut Request) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        println!("{req:#?}");
        println!("\n\
        ==================\n\
        [payload formated]\n\
        {}\n\
        ==================\n\
        ",
            std::str::from_utf8(req.payload().unwrap_or_default()).unwrap(),
        );
        async {Ok(())}
    }
}

struct ForceEmptyOK;
impl BackFang for ForceEmptyOK {
    type Error = std::convert::Infallible;
    fn bite(&self, res: &mut Response, _req: &Request) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        if _req.method().isGET() && _req.path() != "/favicon.ico" {
            // pass
        } else {
            *res = Response::OK();
        }
        async {Ok(())}
    }
}

#[tokio::main]
async fn main() {
    Ohkami::new(()).howl_with((
        RequestLogger,
        ForceEmptyOK,
    ), "localhost:5050").await
}
