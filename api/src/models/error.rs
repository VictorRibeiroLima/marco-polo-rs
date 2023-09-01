use core::fmt;

use actix_web::{
    error::{JsonPayloadError, QueryPayloadError, ResponseError},
    http::StatusCode,
    HttpResponse,
};

use marco_polo_rs_core::internals::video_platform::errors::HeathCheckError;
use serde::{Deserialize, Serialize};

use crate::mail::MailError;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppErrorResponse {
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppErrorType {
    BadRequest,
    InternalServerError,
    NotFound,
    Unauthorized,
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl ResponseError for AppErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}

#[derive(Debug)]
pub struct AppError {
    pub message: String,
    pub error_type: AppErrorType,
}

#[allow(dead_code)]
impl AppError {
    pub fn new(error_type: AppErrorType, message: String) -> Self {
        return Self {
            error_type,
            message,
        };
    }
    pub fn bad_request(message: String) -> Self {
        return Self::new(AppErrorType::BadRequest, message);
    }

    pub fn internal_server_error() -> Self {
        return Self::new(
            AppErrorType::InternalServerError,
            "Internal Server Error".to_string(),
        );
    }

    pub fn not_found(message: String) -> Self {
        return Self::new(AppErrorType::NotFound, message);
    }

    pub fn unauthorized(message: String) -> Self {
        return Self::new(AppErrorType::Unauthorized, message);
    }

    fn message(&self) -> String {
        self.message.clone()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::BadRequest => actix_web::http::StatusCode::BAD_REQUEST,
            AppErrorType::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            AppErrorType::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let errors = self
            .message
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(String::from)
            .collect::<Vec<String>>();
        HttpResponse::build(self.status_code()).json(AppErrorResponse { errors })
    }
}

impl From<Vec<AppError>> for AppError {
    fn from(value: Vec<AppError>) -> Self {
        let mut errors = vec![];

        // We are using BadRequest as a umbrella error type
        let mut error_type = AppErrorType::BadRequest;
        for error in value {
            // If any error is InternalServerError, we should return it
            if error.error_type == AppErrorType::InternalServerError {
                error_type = AppErrorType::InternalServerError;
            }
            errors.push(error.message);
        }
        return Self::new(AppErrorType::BadRequest, errors.join("\n"));
    }
}

impl From<Box<dyn std::error::Error + Sync + Send>> for AppError {
    fn from(value: Box<dyn std::error::Error + Sync + Send>) -> Self {
        return Self::new(AppErrorType::InternalServerError, value.to_string());
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        return Self::new(AppErrorType::InternalServerError, value.to_string());
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        return Self::new(AppErrorType::InternalServerError, value.to_string());
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        match value {
            sqlx::Error::RowNotFound => {
                return Self::new(AppErrorType::NotFound, value.to_string());
            }
            _ => {
                println!("{:?}", value);
                return Self::new(AppErrorType::InternalServerError, value.to_string());
            }
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(value: validator::ValidationErrors) -> Self {
        return Self::new(AppErrorType::BadRequest, value.to_string());
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(value: bcrypt::BcryptError) -> Self {
        return Self::new(AppErrorType::InternalServerError, value.to_string());
    }
}

impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        return Self::new(AppErrorType::InternalServerError, value.to_string());
    }
}

impl From<JsonPayloadError> for AppError {
    fn from(value: JsonPayloadError) -> Self {
        return Self::new(AppErrorType::BadRequest, value.to_string());
    }
}

impl From<QueryPayloadError> for AppError {
    fn from(value: QueryPayloadError) -> Self {
        return Self::new(AppErrorType::BadRequest, value.to_string());
    }
}

impl From<MailError> for AppError {
    fn from(value: MailError) -> Self {
        return Self::new(AppErrorType::InternalServerError, value.to_string());
    }
}

impl From<HeathCheckError<'_>> for AppError {
    fn from(value: HeathCheckError) -> Self {
        match value {
            HeathCheckError::ChannelWrongAuthType(_) => {
                return Self::new(AppErrorType::InternalServerError, value.to_string());
            }
            _ => {
                return Self::new(AppErrorType::BadRequest, value.to_string());
            }
        }
    }
}
