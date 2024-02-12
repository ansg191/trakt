use crate::error::ApiError;

pub trait Response: Sized {
    fn try_from_http_response<T: AsRef<[u8]>>(
        response: http::Response<T>,
    ) -> Result<Self, FromHttpError>;
}

pub trait PaginatedResponse: Response {
    type Item;

    fn items(&self) -> &[Self::Item];

    fn next_page(&self) -> Option<usize>;
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
