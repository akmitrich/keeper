use crate::data::mongo_handler::MongoHandler;
use actix_web::{http::StatusCode, post, web, HttpResponse};

#[post("/upload/{namespace}/{id}")]
async fn upload(
    path: web::Path<(String, String)>,
    body: web::Json<serde_json::Value>,
    handler: web::Data<MongoHandler>,
) -> crate::Result<HttpResponse> {
    let db = handler.client.database(path.0.as_str());
    let scenario = db.collection::<serde_json::Value>(path.1.as_str());
    let cursor = scenario.insert_one(body.into_inner(), None).await?;
    println!("Uploaded: {:?}", cursor.inserted_id);
    Ok(HttpResponse::new(StatusCode::ACCEPTED))
}
