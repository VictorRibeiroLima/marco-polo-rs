use std::fmt;

use actix_web::ResponseError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AppErrorType {
    BadRequest,
    InternalServerError,
    NotFound,
    Unauthorized,
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{:?}", self);
    }
}

impl ResponseError for AppErrorType {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            AppErrorType::BadRequest => actix_web::http::StatusCode::BAD_REQUEST,
            AppErrorType::InternalServerError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            AppErrorType::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        return actix_web::HttpResponse::build(self.status_code()).json(self.to_string());
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppError {
    pub error_type: AppErrorType,
    pub message: String,
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
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self);
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        return self.error_type.status_code();
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        return actix_web::HttpResponse::build(self.status_code()).json(self.to_string());
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(value: Box<dyn std::error::Error>) -> Self {
        println!("Boxed Error: {}", value);
        return Self::new(AppErrorType::InternalServerError, value.to_string());
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        println!("JSON Error: {}", value);
        return Self::new(AppErrorType::InternalServerError, value.to_string());
    }
}

impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        println!("Request Error: {}", value);
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
                println!("SQLx Error: {}", value);
                return Self::new(AppErrorType::InternalServerError, value.to_string());
            }
        }
    }
}
