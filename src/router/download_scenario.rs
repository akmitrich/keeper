use crate::data::mongo_handler::MongoHandler;
use actix_web::{get, web};
use mongodb::{
    bson::{doc, Document},
    options::FindOneOptions,
    Collection,
};

#[get("/{namespace}/{id}/{version}")]
pub async fn download(
    path: web::Path<(String, String, String)>,
    handler: web::Data<MongoHandler>,
) -> crate::Result<web::Json<serde_json::Value>> {
    let (namespace, id, version) = path.into_inner();
    let db = handler.client.database(&namespace);
    let coll = db.collection::<Document>(&id);
    let result = match version.as_str() {
        "__last__" => get_last(coll).await?,
        _ => restore_value(coll, version).await?,
    };
    Ok(web::Json(result))
}

pub async fn restore_value(
    coll: Collection<Document>,
    id: String,
) -> crate::Result<serde_json::Value> {
    let fetched = coll
        .find_one(doc! {"_id": id.clone()}, None)
        .await?
        .ok_or_else(|| value_not_found_err(coll.name(), &id))?;
    extract_ts_value(fetched).ok_or_else(|| value_not_found_err(coll.name(), &id))
}

async fn get_last(coll: Collection<Document>) -> crate::Result<serde_json::Value> {
    let options = FindOneOptions::builder().sort(doc! {"_ts": -1}).build();
    let doc = coll
        .find_one(None, options)
        .await?
        .ok_or_else(|| value_not_found_err(coll.name(), "last"))?;
    extract_ts_value(doc).ok_or_else(|| value_not_found_err(coll.name(), "last"))
}

fn value_not_found_err(name: &str, id: &str) -> crate::Error {
    crate::Error::ValueNotFound(format!("{}:{}", name, id))
}

fn extract_ts_value(found: Document) -> Option<serde_json::Value> {
    let version = found.get("_id")?.as_str()?.to_owned();
    let ts = found.get("_ts")?.as_f64()?;
    let doc = found.get("value")?;
    let value = serde_json::to_value(doc).ok()?;
    Some(serde_json::json!({
        "version": version,
        "timestamp": ts,
        "value": value,
    }))
}
