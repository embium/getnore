use axum::{http::StatusCode, response::IntoResponse, Json};
use bb8_redis::{bb8::RunError, redis::RedisError};
use serde_json::json;
use thiserror::Error;
use tokio::task::JoinError;
use validator::ValidationErrors;

// Define a more structured error response body
#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
    pub success: bool,
}

// Define an AppError enum that scales
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error occurred: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("Validation failed: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error("Redis connection error: {0}")]
    RedisError(#[from] RedisError),

    #[error("Redis pool connection error: {0}")]
    RunRedisError(#[from] RunError<RedisError>),

    #[error("Invalid header value: {0}")]
    FailedToSetHeader(#[from] axum::http::header::InvalidHeaderValue),

    #[error("Authentication required")]
    Unauthorized,

    #[error("Authentication failed: {0}")]
    UnauthorizedError(String),

    #[error("Session has expired")]
    SessionExpired,

    #[error("Refresh token has expired")]
    RefreshTokenExpired,

    #[error("JSON serialization error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("JWT token error: {0}")]
    JWTError(#[from] jsonwebtoken::errors::Error),

    #[error("Processing error: {0}")]
    ProcessError(String),

    #[error("Authorization error: {0}")]
    CasbinError(#[from] casbin::Error),

    #[error("File system error: {0}")]
    TokioFsError(#[from] tokio::io::Error),

    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Requested resource not found")]
    ResourceNotFound,

    #[error("Resource already exists: {0}")]
    ResourceExist(String),

    #[error("Invalid OAuth provider")]
    InvalidOauthProvider,

    #[error("HTTP client error: {0}")]
    HttpClientError(#[from] reqwest::Error),

    #[error("OAuth2 authorization failed")]
    Oauth2FailedToAuthorize,

    #[error("Invalid authentication token")]
    InvalidToken,

    #[error("Email address already registered")]
    UserEmailAlreadyExist,

    #[error("An account with email {0} already exists. Please sign in with your email and password instead.")]
    AccountAlreadyExistsWithEmail(String),

    #[error("No user found with email address: {0}")]
    UserNotExist(String),

    #[error("Access denied. You do not have permission to perform this action.")]
    Forbidden,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_code, message) = match &self {
            AppError::ProcessError(value) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "process_error".to_string(),
                format!("Unable to process request: {}", value),
            ),
            AppError::ValidationError(_) => (
                StatusCode::BAD_REQUEST,
                "validation_error".to_string(),
                "Request validation failed. Please check your input and try again.".to_string(),
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "unauthorized".to_string(),
                "Authentication required. Please sign in to continue.".to_string(),
            ),
            AppError::JWTError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "jwt_error".to_string(),
                "Token validation failed. Please try again.".to_string(),
            ),
            AppError::SqlxError(err) => match err {
                sqlx::Error::RowNotFound => (
                    StatusCode::NOT_FOUND,
                    "resource_not_found".to_string(),
                    "The requested resource could not be found.".to_string(),
                ),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error".to_string(),
                "A database error occurred. Please try again later.".to_string(),
                ),
            },
            AppError::ResourceNotFound => (
                StatusCode::NOT_FOUND,
                "resource_not_found".to_string(),
                "The requested resource could not be found.".to_string(),
            ),
            AppError::ResourceExist(value) => (
                StatusCode::BAD_REQUEST,
                "resource_exist".to_string(),
                format!("Resource already exists: {}", value),
            ),
            AppError::CasbinError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "casbin_error".to_string(),
                format!("Authorization error: {}", err),
            ),
            AppError::InvalidOauthProvider => (
                StatusCode::BAD_REQUEST,
                "invalid_oauth_provider".to_string(),
                "Invalid OAuth provider. Please try a different authentication method.".to_string(),
            ),
            AppError::Oauth2FailedToAuthorize => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "oauth2_failed_to_authorize".to_string(),
                "OAuth2 authorization failed. Please try again.".to_string(),
            ),
            AppError::UnauthorizedError(value) => (
                StatusCode::UNAUTHORIZED,
                "unauthorized".to_string(),
                format!("Authentication failed: {}", value),
            ),
            AppError::SessionExpired => (
                StatusCode::UNAUTHORIZED,
                "access_token_expired".to_string(),
                "Your session has expired. Please sign in again.".to_string(),
            ),
            AppError::RefreshTokenExpired => (
                StatusCode::UNAUTHORIZED,
                "refresh_token_expired".to_string(),
                "Your session has permanently expired. Please sign in again.".to_string(),
            ),
            AppError::UserEmailAlreadyExist => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "user_email_already_exist".to_string(),
                "An account with this email address already exists.".to_string(),
            ),
            AppError::AccountAlreadyExistsWithEmail(value) => (
                StatusCode::CONFLICT,
                "account_already_exists_with_email".to_string(),
                format!("Account with email {} already exists. Please log in with your email and password instead.", value),
            ),
            AppError::UserNotExist(value) => (
                StatusCode::BAD_REQUEST,
                "user_not_exist".to_string(),
                format!("No user found with email address: {value}"),
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "forbidden".to_string(),
                "Access denied. You do not have permission to perform this action.".to_string(),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_server_error".to_string(),
                "An internal server error occurred. Please try again later.".to_string(),
            ),
        };

        let body = Json(json!({
            "error_code": error_code,
            "message": message,
            "success": false,
        }));

        (status, body).into_response()
    }
}

// Convert specific errors into AppError variants
impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::ProcessError(value.to_string())
    }
}

impl From<JoinError> for AppError {
    fn from(value: JoinError) -> Self {
        AppError::ProcessError(value.to_string())
    }
}
