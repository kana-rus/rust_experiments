#![feature(try_trait_v2)]
use std::{ops::FromResidual, convert::Infallible};

struct Response;

/*
    the trait
        `FromResidual<Result<Infallible, Response>>`
    is not implemented for
        `Response`
*/

impl FromResidual<Result<Infallible, Response>> for Response {
    fn from_residual(residual: Result<Infallible, Response>) -> Self {
        unsafe { residual.unwrap_err_unchecked() }
    }
}

fn main() {
    fn make_result() -> Result<String, Response> {
        Ok(format!("Hello, world!"))
    }

    fn handler() -> Response {
        let _ = make_result()?;
        Response
    }
}
