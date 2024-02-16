use std::{num::ParseIntError, str::FromStr};

use http::{header::AsHeaderName, HeaderMap, StatusCode};
use serde::Serialize;

use crate::error::{ApiError, DeserializeError, FromHttpError, HeaderError};

/// `Pagination` struct is used to specify the page number and the maximum number of items to be shown per page.
///
/// Default values are `page = 1` and `limit = 10`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize)]
pub struct Pagination {
    pub page: usize,
    pub limit: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self::DEFAULT
    }
}

impl Pagination {
    const DEFAULT: Self = Self { page: 1, limit: 10 };

    #[inline]
    #[must_use]
    pub const fn new(page: usize, limit: usize) -> Self {
        Self { page, limit }
    }
}

/// `PaginationResponse` struct is used to store the paginated response from the API.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    pub current_page: usize,
    pub items_per_page: usize,
    pub total_pages: usize,
    pub total_items: usize,
}

impl<T> PaginationResponse<T> {
    /// Create a new `PaginationResponse` instance from items and Trakt.tv API response headers.
    ///
    /// # Errors
    ///
    /// Returns a `DeserializeError` if the headers are missing or if the header values are not valid.
    pub fn from_headers(items: Vec<T>, map: &HeaderMap) -> Result<Self, DeserializeError> {
        let current_page = parse_from_header(map, "X-Pagination-Page")?;
        let items_per_page = parse_from_header(map, "X-Pagination-Limit")?;
        let total_pages = parse_from_header(map, "X-Pagination-Page-Count")?;
        let total_items = parse_from_header(map, "X-Pagination-Item-Count")?;

        Ok(Self {
            items,
            current_page,
            items_per_page,
            total_pages,
            total_items,
        })
    }

    #[inline]
    #[must_use]
    pub const fn next_page(&self) -> Option<Pagination> {
        if self.current_page < self.total_pages {
            Some(Pagination::new(self.current_page + 1, self.items_per_page))
        } else {
            None
        }
    }
}

/// Helper function to parse a header value to an integer.
///
/// # Errors
///
/// Returns a `DeserializeError` if the header is missing, if the header value is not a valid
/// string, or if the string value cannot be parsed to an integer.
pub fn parse_from_header<T, K>(map: &HeaderMap, key: K) -> Result<T, DeserializeError>
where
    T: FromStr<Err = ParseIntError>,
    K: AsHeaderName,
{
    map.get(key)
        .ok_or(HeaderError::MissingHeader)?
        .to_str()
        .map_err(HeaderError::ToStrError)?
        .parse()
        .map_err(DeserializeError::ParseInt)
}

/// Helper function to handle the response body from the API.
///
/// Will check if the response has the expected status code and will try to deserialize the
/// response body.
///
/// # Errors
///
/// Returns a `FromHttpError` if the response status code is not the expected one or if the body
/// failed to be deserialized.
pub fn handle_response_body<B, T>(
    response: &http::Response<B>,
    expected: StatusCode,
) -> Result<T, FromHttpError>
where
    B: AsRef<[u8]>,
    T: serde::de::DeserializeOwned,
{
    if response.status() == expected {
        Ok(serde_json::from_slice(response.body().as_ref()).map_err(DeserializeError::Json)?)
    } else {
        Err(FromHttpError::Api(ApiError::from(response.status())))
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderValue;

    use super::*;

    #[test]
    fn test_parse_from_header() {
        let mut map = HeaderMap::new();
        map.insert("B", HeaderValue::from_bytes(b"hello\xfa").unwrap());
        map.insert("C", HeaderValue::from_static("hello"));
        map.insert("D", HeaderValue::from_static("10"));

        assert!(matches!(
            parse_from_header::<u32, _>(&map, "A"),
            Err(DeserializeError::Header(HeaderError::MissingHeader))
        ));
        assert!(matches!(
            parse_from_header::<u32, _>(&map, "B"),
            Err(DeserializeError::Header(HeaderError::ToStrError(_)))
        ));
        assert!(matches!(
            parse_from_header::<u32, _>(&map, "C"),
            Err(DeserializeError::ParseInt(_))
        ));
        assert_eq!(parse_from_header::<u32, _>(&map, "D").unwrap(), 10);
    }

    #[test]
    fn test_handle_response_body_ok() {
        let response = http::Response::builder()
            .status(StatusCode::OK)
            .body(b"\"hello\"")
            .unwrap();
        assert_eq!(
            handle_response_body::<_, String>(&response, StatusCode::OK).unwrap(),
            "hello"
        );
    }

    #[test]
    fn test_handle_response_body_bad_request() {
        let response = http::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(b"\"hello\"")
            .unwrap();
        assert!(matches!(
            handle_response_body::<_, String>(&response, StatusCode::OK),
            Err(FromHttpError::Api(ApiError::BadRequest))
        ));
    }

    #[test]
    fn test_handle_response_body_deserialize_error() {
        let response = http::Response::builder()
            .status(StatusCode::OK)
            .body(b"\"hello\xfa\"")
            .unwrap();
        assert!(matches!(
            handle_response_body::<_, String>(&response, StatusCode::OK),
            Err(FromHttpError::Deserialize(DeserializeError::Json(_)))
        ));
    }
}
