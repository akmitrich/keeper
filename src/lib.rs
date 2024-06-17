pub mod data;
mod error;
// mod router;

use actix_web::{dev::Server, web, App, HttpServer};
pub use error::*;

pub fn server_from_listener(lst: std::net::TcpListener) -> Result<Server> {
    let start_time = web::Data::new(data::start_time::StartTime::new());
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&start_time))
            .app_data(web::JsonConfig::default().limit(1 << 32))
        // .configure(router::main_route_factory)
    })
    .listen(lst)?
    .run())
}
