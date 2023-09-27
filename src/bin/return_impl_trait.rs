trait IntoResponse {}
struct Response;
impl IntoResponse for Response {}
impl IntoResponse for Result<Response, Response> {}

fn make_result() -> Result<Response, Response> {
    Ok(Response)
}

// fn f() -> impl IntoResponse {
//     let res = make_result()?;
//     res
// }

fn main() {

}
