use actix_web::ResponseError;
use derive_more::From;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Custom(String),

    // -- Externals
    #[from]
    Io(std::io::Error), // as example
    #[from]
    NoCurrentRuntime(tokio::runtime::TryCurrentError),
    #[from]
    SystemTimeStamp(std::time::SystemTimeError),
    #[from]
    MongoDBFailed(mongodb::error::Error),
    #[from]
    BsonSerialization(mongodb::bson::ser::Error),
    #[from]
    JsonSerialization(serde_json::Error),

    // -- Internals
    ValueNotFound(String),
    RouteNotFound(serde_json::Value),
}

// region:    --- Custom

impl Error {
    pub fn custom(val: impl std::fmt::Display) -> Self {
        Self::Custom(val.to_string())
    }
}

impl From<&str> for Error {
    fn from(val: &str) -> Self {
        Self::Custom(val.to_string())
    }
}

// endregion: --- Custom

// region:    --- Error Boilerplate

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

// endregion: --- Error Boilerplate

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Error::MongoDBFailed(_) => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            Error::ValueNotFound(_) => actix_web::http::StatusCode::NOT_ACCEPTABLE,
            Error::RouteNotFound(_) => actix_web::http::StatusCode::NOT_FOUND,
            _ => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
