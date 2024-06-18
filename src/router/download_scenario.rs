use crate::data::mongo_handler::MongoHandler;
use actix_web::{get, web};
use mongodb::bson::Document;

#[get("/{namespace}/{id}/{version}")]
pub async fn download(
    path: web::Path<(String, String, String)>,
    handler: web::Data<MongoHandler>,
) -> crate::Result<web::Json<serde_json::Value>> {
    let (namespace, id, version) = path.into_inner();
    let db = handler.client.database(&namespace);
    let coll = db.collection::<Document>(&id);
    let (ts, body) = handler.restore_value(coll, version).await?;
    Ok(web::Json(
        serde_json::json!({"timestamp": ts, "value": body}),
    ))
}
