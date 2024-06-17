#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    keeper::server_from(
        std::net::TcpListener::bind("0.0.0.0:8000")?,
        keeper::data::connect_to().await,
    )?
    .await?;
    Ok(())
}
