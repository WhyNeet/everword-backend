use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("An internal error occured.")]
    Internal,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(json!({ "message": self.to_string() }))
    }
}

pub type AppResult<T> = Result<T, AppError>;
