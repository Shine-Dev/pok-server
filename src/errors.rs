// src/errors.rs

use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ApiError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "JWKSFetchError")] 
    JWKSFetchError,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalServerError | Self::JWKSFetchError => { 
                HttpResponse::InternalServerError().json("Internal Server Error, try later.") 
            },
            Self::BadRequest(message) => HttpResponse::BadRequest().json(message),
        }
    }
}