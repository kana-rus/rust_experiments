use worker::*;

async fn __(req: Request) {
    req.form_data().await
}

fn main() {

}
