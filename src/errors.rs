use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "KeyFetchError")] 
    KeyFetchFetchError,

    #[display(fmt = "AuthError")] 
    AuthError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::BadRequest(message) => HttpResponse::BadRequest().json(message),
            Self::AuthError => HttpResponse::Unauthorized().finish(),
            _  =>  HttpResponse::InternalServerError().finish(),
        }
    }
}