use crate::data::mongo_handler::MongoHandler;
use actix_web::{http::StatusCode, post, web, HttpResponse};
use mongodb::{
    bson::{self, doc, Document},
    Collection,
};

#[post("/upload/{namespace}/{id}/{version}")]
async fn upload(
    path: web::Path<(String, String, String)>,
    body: web::Json<serde_json::Value>,
    handler: web::Data<MongoHandler>,
) -> crate::Result<HttpResponse> {
    log::trace!("Store into: {:?}", path);
    let (namespace, id, version) = path.into_inner();
    let db = handler.client.database(&namespace);
    let coll = db.collection::<Document>(&id);
    keep_value(coll, version, &body).await?;
    Ok(HttpResponse::new(StatusCode::ACCEPTED))
}

pub async fn keep_value(
    coll: Collection<Document>,
    id: String,
    value: &serde_json::Value,
) -> crate::Result<()> {
    let inserted = coll
        .insert_one(
            doc! {
                "_id": id,
                "_ts": std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH)?.as_secs_f64(),
                "value": bson::to_bson(value)?,
            },
            None,
        )
        .await?;
    log::info!("Inserted {:?}", inserted.inserted_id);
    Ok(())
}
