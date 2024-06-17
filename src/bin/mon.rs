#[tokio::main]
async fn main() {
    keeper::data::connect_to().await;
}
