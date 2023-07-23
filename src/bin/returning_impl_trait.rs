#![feature(try_trait_v2)]
use std::{ops::FromResidual, convert::Infallible};


trait Response: Sized {fn into_response(self) -> ResponseImpl;}
struct ResponseImpl {
    status:  u16,
    content: Option<String>,
}

struct OkResponse(Option<String>);
impl Response for OkResponse {fn into_response(self) -> ResponseImpl {
    ResponseImpl { status: 200, content: self.0 }
}}
impl OkResponse {
    fn new() -> Self {Self(None)}

    fn text<Text: ToString>(mut self, text: Text) -> Self {
        self.0.replace(text.to_string());
        self
    }
    fn json<JSON: ToString>(mut self, text: JSON) -> Self {
        self.0.replace(text.to_string());
        self
    }
}

struct CreatedResponse(String);
impl Response for CreatedResponse {
    fn into_response(self) -> ResponseImpl {
        ResponseImpl { status: 201, content: Some(self.0) }
    }
}
impl CreatedResponse {
    fn new<Entity: ToString>(entity: Entity) -> Self {Self(entity.to_string())}
}

const _: (/*===== KORE =====*/) = {
    impl FromResidual<Result<Infallible, OkResponse>> for OkResponse {
        fn from_residual(residual: Result<Infallible, OkResponse>) -> Self {
            todo!()
        }
    }
    impl FromResidual<Result<Infallible, CreatedResponse>> for OkResponse {
        fn from_residual(residual: Result<Infallible, CreatedResponse>) -> Self {
            todo!()
        }
    }
    impl FromResidual<Result<Infallible, OkResponse>> for CreatedResponse {
        fn from_residual(residual: Result<Infallible, OkResponse>) -> Self {
            todo!()
        }
    }
    impl FromResidual<Result<Infallible, CreatedResponse>> for CreatedResponse {
        fn from_residual(residual: Result<Infallible, CreatedResponse>) -> Self {
            todo!()
        }
    }
};


struct Context;
#[allow(non_snake_case)]
impl Context {
    fn OK(&self) -> OkResponse {OkResponse::new()}
    fn Created<Entity: ToString>(&self, entity: Entity) -> CreatedResponse {CreatedResponse::new(entity)}
}


trait IntoHandler<Res:Response>: Sized {
    fn into_handler(self) {}
}
impl<F: Fn(Context)->Res, Res: Response> IntoHandler<Res> for F {}


fn main() {
    fn get_num() -> Result<usize, std::io::Error> {
        Ok(42)
    }
    fn get_vec() -> Result<Vec<usize>, std::io::Error> {
        Ok(vec![42])
    }

    fn handler_1(c: Context) -> impl Response {
        let num = get_num()
            .map_err(|e| c.OK().text(format!("Error: {e}")))?;

        c.Created(format!("num is {num}"))
    }
    fn handler_2(c: Context) -> impl Response {
        let vec = get_vec()
            .map_err(|_| c.OK())?;

        c.Created(format!("vec is {vec:?}"))
    }
}
