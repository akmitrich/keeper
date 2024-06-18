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

    pub async fn restore_value(
        &self,
        coll: Collection<Document>,
        id: String,
    ) -> crate::Result<(f64, serde_json::Value)> {
        let fetched = coll
            .find_one(doc! {"_id": id.clone()}, None)
            .await?
            .ok_or_else(|| value_not_found_err(coll.name(), &id))?;
        let ts = fetched
            .get("_ts")
            .ok_or_else(|| value_not_found_err(coll.name(), &id))?
            .as_f64()
            .ok_or_else(|| value_not_found_err(coll.name(), &id))?;
        let doc = fetched
            .get("value")
            .ok_or_else(|| value_not_found_err(coll.name(), &id))?;
        let value = serde_json::to_value(doc)?;

        Ok((ts, value))
    }
}

fn value_not_found_err(name: &str, id: &str) -> crate::Error {
    crate::Error::ValueNotFound(format!("{}:{}", name, id))
}
