use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use jsonwebtoken::errors::Error as JwtError;
use sqlx::Error as SqlxError;

#[derive(Display, Debug)]
pub enum ServiceError {
    #[display(fmt = "Unauthorized: {}", _.0)]
    Unauthorized(&'static str),
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::Unauthorized(ref message) => HttpResponse::Unauthorized().json(message),
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
        }
    }
}

impl From<SqlxError> for ServiceError {
    fn from(_error: SqlxError) -> Self {
        ServiceError::InternalServerError
    }
}

impl From<JwtError> for ServiceError {
    fn from(_error: JwtError) -> Self {
        ServiceError::Unauthorized("invalid token")
    }
}
