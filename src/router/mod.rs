use actix_web::{get, web, HttpRequest};

use crate::data::start_time::StartTime;

pub fn main_route_factory(config: &mut web::ServiceConfig) {
    config
        .service(health_check)
        .default_service(web::route().to(not_found));
}

#[get("/check")]
async fn health_check(
    start_time: web::Data<StartTime>,
) -> crate::Result<web::Json<serde_json::Value>> {
    let start: chrono::DateTime<chrono::Local> = start_time.start_time().into();
    let alive = start_time.alive();
    Ok(web::Json(serde_json::json!({
        "started at": format!("{}",start.format("%c")),
        "description": format!("I'm alive for {:?}", alive)
    })))
}

async fn not_found(req: HttpRequest) -> crate::Result<web::Json<()>> {
    Err(crate::Error::RouteNotFound(serde_json::json!({
        "request": format!("Request: {:?}.", req),
        "description": "Не зная броду, не лезь в воду."
    })))
}
