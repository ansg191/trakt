use crate::ApiError;

/// A trait for converting an HTTP response into a result of `Self`.
pub trait Response: Sized {
    /// Converts an HTTP response into a result of `Self`, where `Self` refers to the implementing type.
    ///
    /// # Arguments
    ///
    /// * `response` - The HTTP response to convert.
    ///
    /// # Errors
    ///
    /// Will error if the HTTP response is not a succeeding status code as determined by the
    /// type or if the response body cannot be deserialized into the implementing type.
    ///
    /// See [`FromHttpError`] for more details.
    fn try_from_http_response<T: AsRef<[u8]>>(
        response: http::Response<T>,
    ) -> Result<Self, FromHttpError>;
}

/// A sub-trait of `Response` for paginated responses.
pub trait PaginatedResponse: Response {
    /// The type of item that the paginated response contains.
    type Item;

    /// Returns a slice of the items in the current page of the paginated response.
    fn items(&self) -> &[Self::Item];

    /// Returns the index of the next page of the paginated response.
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
