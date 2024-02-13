//! Error types for the API.

use http::{header::InvalidHeaderValue, StatusCode};

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("Bad Request")]
    BadRequest,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Not Found")]
    NotFound,
    #[error("Resource Already Exists")]
    AlreadyExists,
    #[error("Resource Expired")]
    Expired,
    #[error("Invalid Content-Type")]
    InvalidContentType,
    #[error("User denied the request")]
    Denied,
    #[error("Account limit exceeded")]
    AccountLimitExceeded,
    #[error("Validation Error")]
    ValidationError,
    #[error("Locked User Account")]
    LockedUserAccount,
    #[error("VIP Only")]
    VipOnly,
    #[error("Rate Limit Exceeded")]
    RateLimitExceeded,
    #[error("Server Error")]
    ServerError,
    #[error("Service Unavailable")]
    ServiceUnavailable,
    #[error("Cloudflare Error")]
    CloudflareError,
    #[error("Unknown Error: {0}")]
    UnknownError(StatusCode),
}

impl From<StatusCode> for ApiError {
    fn from(value: StatusCode) -> Self {
        match value.as_u16() {
            400 => Self::BadRequest,
            401 => Self::Unauthorized,
            403 => Self::Forbidden,
            404 => Self::NotFound,
            409 => Self::AlreadyExists,
            410 => Self::Expired,
            412 => Self::InvalidContentType,
            418 => Self::Denied,
            420 => Self::AccountLimitExceeded,
            422 => Self::ValidationError,
            423 => Self::LockedUserAccount,
            426 => Self::VipOnly,
            429 => Self::RateLimitExceeded,
            500 => Self::ServerError,
            502..=504 => Self::ServiceUnavailable,
            520..=522 => Self::CloudflareError,
            _ => Self::UnknownError(value),
        }
    }
}

/// Error type for converting a request into an HTTP request.
#[derive(Debug, thiserror::Error)]
pub enum IntoHttpError {
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid Header Value: {0}")]
    Header(#[from] InvalidHeaderValue),
    #[error("HTTP Error: {0}")]
    Http(#[from] http::Error),
    #[error("Url params error: {0}")]
    UrlParams(#[from] UrlError),
    #[error("Query params error: {0}")]
    QueryParams(#[from] serde_urlencoded::ser::Error),
    #[error("Missing oauth token")]
    MissingToken,
}

#[derive(Debug, thiserror::Error)]
pub enum FromHttpError {
    #[error("API Error: {0}")]
    Api(#[from] ApiError),
    #[error("Deserialize Error: {0}")]
    Deserialize(#[from] DeserializeError),
}

#[derive(Debug, thiserror::Error)]
pub enum DeserializeError {
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Header Error: {0}")]
    Header(#[from] HeaderError),
    #[error("Integer Parse Error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

#[derive(Debug, thiserror::Error)]
pub enum HeaderError {
    #[error("Invalid Header Value: {0}")]
    ToStrError(#[from] http::header::ToStrError),
    #[error("Missing Header")]
    MissingHeader,
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum UrlError {
    #[error("{0}")]
    Message(String),
    #[error("Top level serializer only supports structs")]
    TopLevel,
    #[error("Invalid endpoint")]
    InvalidEndpoint,
    #[error("Value not supported")]
    ValueNotSupported,
    #[error("Key not found: {0}")]
    KeyNotFound(&'static str),
    #[error("Unfilled field: {0}")]
    UnfilledField(String),
}

impl serde::ser::Error for UrlError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}
