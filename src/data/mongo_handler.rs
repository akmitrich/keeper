use mongodb::{
    bson::{self, doc, Document},
    Collection,
};

#[derive(Debug)]
pub struct MongoHandler {
    pub client: mongodb::Client,
}

impl MongoHandler {
    pub async fn connect_localhost() -> Self {
        let connection_string = format!("mongodb://localhost:{}", 27017);
        let client = mongodb::Client::with_uri_str(connection_string)
            .await
            .unwrap();
        Self { client }
    }

    pub async fn keep_value(
        &self,
        coll: Collection<Document>,
        id: String,
        value: &serde_json::Value,
    ) -> crate::Result<()> {
        let inserted = coll
            .insert_one(
                doc! {
                    "_id": id,
                    "_ts": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64(),
                    "value": bson::to_bson(value)?,
                },
                None,
            )
            .await?;
        Ok(())
    }
}
