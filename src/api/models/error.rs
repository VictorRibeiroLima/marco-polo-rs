use core::fmt;

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};

use serde::Serialize;

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

#[derive(Debug)]
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
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
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
