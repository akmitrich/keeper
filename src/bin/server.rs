#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "actix_web=debug, actix_server=debug, server=trace, keeper=trace",
        );
    }
    env_logger::init();
    log::info!("Server started at {:?}", std::time::SystemTime::now());
    keeper::server_from(
        std::net::TcpListener::bind("0.0.0.0:8000")?,
        keeper::data::connect_to().await,
    )?
    .await?;
    Ok(())
}
