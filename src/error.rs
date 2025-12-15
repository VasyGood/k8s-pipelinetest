use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde_json::json;

pub enum AppError {
    MongoDB(mongodb::error::Error),
    Bson(mongodb::bson::oid::Error),
    NotFound(String),
    Validation(validator::ValidationErrors),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::MongoDB(error) => (StatusCode::INTERNAL_SERVER_ERROR, error.to_string()),
            AppError::Bson(error) => (StatusCode::BAD_REQUEST, error.to_string()),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            AppError::Validation(errors) => (StatusCode::BAD_REQUEST, errors.to_string()),
        };        

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

impl From<mongodb::error::Error> for AppError {
    fn from(error: mongodb::error::Error) -> Self {
        AppError::MongoDB(error)
    }
}


impl From<mongodb::bson::oid::Error> for AppError {
    fn from(error: mongodb::bson::oid::Error) -> Self {
        AppError::Bson(error)
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        AppError::Validation(errors)
    }
}