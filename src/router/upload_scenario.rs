use crate::data::mongo_handler::MongoHandler;
use actix_web::{http::StatusCode, post, web, HttpResponse};
use mongodb::bson::Document;

#[post("/upload/{namespace}/{id}/{version}")]
async fn upload(
    path: web::Path<(String, String, String)>,
    body: web::Json<serde_json::Value>,
    handler: web::Data<MongoHandler>,
) -> crate::Result<HttpResponse> {
    let (namespace, id, version) = path.into_inner();
    let db = handler.client.database(&namespace);
    let coll = db.collection::<Document>(&id);
    handler.keep_value(coll, version, &body).await?;
    Ok(HttpResponse::new(StatusCode::ACCEPTED))
}
