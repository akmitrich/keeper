pub mod data;
mod error;
mod router;

use actix_web::{dev::Server, web, App, HttpServer};
use data::mongo_handler::MongoHandler;
pub use error::*;

pub fn server_from(lst: std::net::TcpListener, handler: MongoHandler) -> Result<Server> {
    let start_time = web::Data::new(data::start_time::StartTime::new());
    let data_handler = web::Data::new(handler);
    Ok(HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&start_time))
            .app_data(web::Data::clone(&data_handler))
            .app_data(web::JsonConfig::default().limit(1 << 32))
            .configure(router::main_route_factory)
    })
    .listen(lst)?
    .run())
}
