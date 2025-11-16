use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Stock insuficiente para el producto")]
    InsufficientStock,

    #[error("Cliente inactivo o no encontrado")]
    InactiveClient,

    #[error("Producto no encontrado o inactivo")]
    ProductNotFound,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::NotFound(_) => HttpResponse::NotFound().json(ErrorResponse {
                error: self.to_string(),
                code: "NOT_FOUND",
            }),
            ApiError::InvalidInput(_) => HttpResponse::BadRequest().json(ErrorResponse {
                error: self.to_string(),
                code: "INVALID_INPUT",
            }),
            ApiError::BusinessRuleViolation(_) => HttpResponse::BadRequest().json(ErrorResponse {
                error: self.to_string(),
                code: "BUSINESS_RULE_VIOLATION",
            }),
            ApiError::InsufficientStock => HttpResponse::BadRequest().json(ErrorResponse {
                error: self.to_string(),
                code: "INSUFFICIENT_STOCK",
            }),
            ApiError::InactiveClient => HttpResponse::BadRequest().json(ErrorResponse {
                error: self.to_string(),
                code: "INACTIVE_CLIENT",
            }),
            ApiError::ProductNotFound => HttpResponse::NotFound().json(ErrorResponse {
                error: self.to_string(),
                code: "PRODUCT_NOT_FOUND",
            }),
            _ => HttpResponse::InternalServerError().json(ErrorResponse {
                error: self.to_string(),
                code: "INTERNAL_ERROR",
            }),
        }
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => ApiError::NotFound("Resource not found".to_string()),
            _ => ApiError::DatabaseError(err.to_string()),
        }
    }
}

#[derive(serde::Serialize, ToSchema)]
pub struct ErrorResponse {
    pub error: String,
    pub code: &'static str,
}

pub type ApiResult<T> = Result<T, ApiError>;
