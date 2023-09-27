use worker::*;

async fn __(mut req: Request) {
    req.form_data().await;
}

fn main() {

}
