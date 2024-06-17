#[derive(Debug)]
pub struct MongoHandler {
    client: mongodb::Client,
}

impl MongoHandler {
    pub async fn connect_localhost() -> Self {
        let connection_string = format!("mongodb://localhost:{}", 27017);
        let client = mongodb::Client::with_uri_str(connection_string)
            .await
            .unwrap();
        Self { client }
    }
}
