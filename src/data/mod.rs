use mongo_handler::MongoHandler;

pub mod mongo_handler;
pub mod start_time;

pub async fn connect_to() -> MongoHandler {
    MongoHandler::connect_localhost().await
}
