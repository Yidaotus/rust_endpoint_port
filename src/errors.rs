use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde_repr::{Serialize_repr};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize_repr)]
#[repr(u8)]
enum ApiStatus {
    OK = 1,
    UNAUTHANTICATED = 2,
    UNAUTHORIZED = 3,
    INVALIDARGUMENT = 4,
    ERROR = 5,
}

#[derive(Serialize)]
struct ApiResponse<T> {
    status: ApiStatus,
    message: String,
    payload: Option<T>,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Validation Error because of {0}")]
    ValidationError(String),
    #[error("Invalid or expired Token!")]
    InvalidToken,
    #[error("Username or Password incorrect!")]
    InvalidUser,
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InvalidToken => StatusCode::UNAUTHORIZED,
            Self::InvalidUser => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let api_status = match self {
            Self::ValidationError(_) => ApiStatus::INVALIDARGUMENT,
            Self::InvalidToken => ApiStatus::UNAUTHORIZED,
            Self::InvalidUser => ApiStatus::ERROR,
        };
        let api_message = self.to_string();
        let api_response = ApiResponse::<()> {
            status: api_status,
            message: api_message,
            payload: None,
        };

        HttpResponse::build(status_code).json(api_response)
    }
}
