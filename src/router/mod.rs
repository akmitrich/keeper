use actix_web::{get, web, HttpRequest, HttpResponse};

use crate::data::start_time::StartTime;

pub fn main_route_factory(config: &mut web::ServiceConfig) {
    config
        .service(health_check)
        .default_service(web::route().to(not_found));
}

#[get("/check")]
async fn health_check(start_time: web::Data<StartTime>) -> HttpResponse {
    let start: chrono::DateTime<chrono::Local> = start_time.start_time().into();
    let alive = start_time.alive();
    HttpResponse::Ok().json(serde_json::json!({
        "started at": format!("{}",start.format("%c")),
        "description": format!("I'm alive for {:?}", alive)
    }))
}

async fn not_found(req: HttpRequest) -> HttpResponse {
    HttpResponse::NotFound().json(serde_json::json!({
        "request": format!("Request: {:?}.", req),
        "description": "Не зная броду, не лезь в воду."
    }))
}
