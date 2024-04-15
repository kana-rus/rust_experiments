use axum::{Router, routing::get};

use std::{pin::Pin, future::Future};
use axum::{body::Body, http::Request};
use tower::{Layer, Service};

use serde::{Serialize, Deserialize};
use axum::extract::{Path, Json};


#[derive(Clone)]
struct Logger;
impl<S: Service<Request<Body>>> Layer<S> for Logger {
    type Service = LoggerService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        LoggerService { inner }
    }
}
#[derive(Clone)]
struct LoggerService<S: Service<Request<Body>>> {
    inner: S,
}
impl<S: Service<Request<Body>>> Service<Request<Body>> for LoggerService<S>
where
    S::Future:   Send + 'static,
    S::Response: std::fmt::Debug,
{
    type Response = S::Response;
    type Error    = S::Error;
    type Future   = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    #[inline]
    fn call(&mut self, req: Request<Body>) -> Self::Future {
        tracing::info!("\n{req:?}");
        let res = self.inner.call(req);
        Box::pin(async move {
            let res = res.await?;
            tracing::info!("\n{res:?}");
            Ok(res)
        })
    }
}


#[derive(Serialize, Deserialize)]
struct Message {
    message: String
}

async fn hello(Path(name): Path<String>) -> Json<Message> {
    Json(Message {
        message: format!("Hello, {name}!")
    })
}


#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    axum::serve(
        tokio::net::TcpListener::bind("localhost:3000").await.unwrap(),
        Router::new()
            .route("/hello/:name", get(hello))
            .layer(Logger)
    ).await.unwrap()
}
