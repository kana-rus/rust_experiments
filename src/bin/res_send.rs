#![feature(try_trait_v2)]
use std::{io::Error, ops::FromResidual, convert::Infallible};


struct Res;
impl FromResidual<Result<Infallible, Res>> for Res {
    fn from_residual(residual: Result<Infallible, Res>) -> Self {
        unsafe { residual.unwrap_err_unchecked() }
    }
}


struct Context(usize);
impl Context {
    fn new() -> Self {
        Self(0)
    }
    fn increment(&mut self) {
        self.0 += 1
    }
}
#[allow(non_snake_case)]
impl Context {
    fn OK        (self) -> Res {Res}
    fn Created   (self) -> Res {Res}
    fn BadRequest(self) -> Res {Res}
}


fn make_result_1() -> Result<usize,  Error> {Ok(42)}
fn make_result_2() -> Result<String, Error> {Ok(format!("Hello!"))}

fn __(c: Context) -> Res {
    make_result_1()
        .map_err(|_| c.BadRequest())?;

    c.OK()
}

