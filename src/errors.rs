use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use diesel::result::{DatabaseErrorKind, Error as DbError};
use std::convert::From;
use uuid::Error as ParseError;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),
    #[display(fmt = "Unauthorized")]
    Unauthorized,
    // #[display(fmt = "NotFound")]
    // NotFound,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            Self::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            Self::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            Self::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            // Self::NotFound => HttpResponse::NotFound().json("Not Found"),
        }
    }
}

impl From<ParseError> for ServiceError {
    fn from(_: ParseError) -> Self {
        Self::BadRequest("Invalid UUID".into())
    }
}

impl From<r2d2::Error> for ServiceError {
    fn from(_: r2d2::Error) -> Self {
        Self::InternalServerError
    }
}

impl From<DbError> for ServiceError {
    fn from(error: DbError) -> Self {
        match error {
            DbError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Self::BadRequest(message);
                }
                Self::InternalServerError
            }
            _ => Self::InternalServerError,
        }
    }
}
