use http::StatusCode;

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
    #[error("Invalid Content-Type")]
    InvalidContentType,
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
            412 => Self::InvalidContentType,
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
