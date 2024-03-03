#[tokio::main]
async fn main() {
    let text = reqwest::get("http://localhost:3000").await.unwrap()
        .text().await.unwrap();

    println!("{text}");
}