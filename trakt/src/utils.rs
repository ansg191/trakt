use http::{header::AsHeaderName, HeaderMap, StatusCode};
use serde::Serialize;

use crate::{error::ApiError, DeserializeError, FromHttpError, HeaderError};

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    pub current_page: usize,
    pub items_per_page: usize,
    pub total_pages: usize,
    pub total_items: usize,
}

impl<T> PaginationResponse<T> {
    pub(crate) fn from_headers(items: Vec<T>, map: &HeaderMap) -> Result<Self, DeserializeError> {
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
    pub const fn next_page(&self) -> Option<usize> {
        if self.current_page < self.total_pages {
            Some(self.current_page + 1)
        } else {
            None
        }
    }
}

pub fn parse_from_header<K: AsHeaderName>(
    map: &HeaderMap,
    key: K,
) -> Result<usize, DeserializeError> {
    map.get(key)
        .ok_or(HeaderError::MissingHeader)?
        .to_str()
        .map_err(HeaderError::ToStrError)?
        .parse()
        .map_err(DeserializeError::ParseInt)
}

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
