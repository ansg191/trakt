use std::{num::ParseIntError, str::FromStr};

use http::{header::AsHeaderName, HeaderMap, StatusCode};
use serde::Serialize;

use crate::{
    error::{ApiError, DeserializeError, FromHttpError, HeaderError, IntoHttpError},
    AuthRequirement, Context, Metadata,
};

/// `Pagination` struct is used to specify the page number and the maximum
/// number of items to be shown per page.
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
    const DEFAULT: Self = Self::new(1, 10);

    #[inline]
    #[must_use]
    pub const fn new(page: usize, limit: usize) -> Self {
        Self { page, limit }
    }
}

/// `PaginationResponse` struct is used to store the paginated response from the
/// API.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct PaginationResponse<T> {
    pub items: Vec<T>,
    pub current_page: usize,
    pub items_per_page: usize,
    pub total_pages: usize,
    pub total_items: usize,
}

impl<T> PaginationResponse<T> {
    /// Create a new `PaginationResponse` instance from items and Trakt.tv API
    /// response headers.
    ///
    /// # Errors
    ///
    /// Returns a `DeserializeError` if the headers are missing or if the header
    /// values are not valid.
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
/// Returns a `DeserializeError` if the header is missing, if the header value
/// is not a valid string, or if the string value cannot be parsed to an
/// integer.
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
/// Will check if the response has the expected status code and will try to
/// deserialize the response body.
///
/// # Errors
///
/// Returns a `FromHttpError` if the response status code is not the expected
/// one or if the body failed to be deserialized.
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

/// Helper function to construct an HTTP request using the given context,
/// metadata, and path/query/body values.
///
/// # Errors
///
/// Returns an `IntoHttpError` if the http request cannot be constructed.
pub fn construct_req<B>(
    ctx: &Context,
    md: &Metadata,
    path: &impl Serialize,
    query: &impl Serialize,
    body: B,
) -> Result<http::Request<B>, IntoHttpError> {
    let url = crate::construct_url(ctx.base_url, md.endpoint, path, query)?;

    let request = http::Request::builder()
        .method(&md.method)
        .uri(url)
        .header("Content-Type", "application/json")
        .header("trakt-api-version", "2")
        .header("trakt-api-key", ctx.client_id);
    let request = match (md.auth, ctx.oauth_token) {
        (AuthRequirement::None, _) | (AuthRequirement::Optional, None) => request,
        (AuthRequirement::Optional | AuthRequirement::Required, Some(token)) => {
            request.header("Authorization", format!("Bearer {token}"))
        }
        (AuthRequirement::Required, None) => {
            return Err(IntoHttpError::MissingToken);
        }
    };
    Ok(request.body(body)?)
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

    #[allow(clippy::cognitive_complexity)]
    #[test]
    fn test_construct_req() {
        let mut ctx = Context {
            base_url: "https://api.trakt.tv",
            client_id: "client id",
            oauth_token: None,
        };
        let mut md = Metadata {
            endpoint: "/test",
            method: http::Method::GET,
            auth: AuthRequirement::None,
        };

        let req = construct_req(&ctx, &md, &(), &(), "body").unwrap();
        assert_eq!(req.method(), &http::Method::GET);
        assert_eq!(req.uri(), "https://api.trakt.tv/test");
        assert_eq!(
            req.headers().get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(req.headers().get("trakt-api-version").unwrap(), "2");
        assert_eq!(req.headers().get("trakt-api-key").unwrap(), "client id");
        assert!(req.headers().get("Authorization").is_none());
        assert_eq!(req.into_body(), "body");

        md.auth = AuthRequirement::Required;
        ctx.oauth_token = Some("token");

        let req = construct_req(&ctx, &md, &(), &(), "body").unwrap();
        assert_eq!(req.method(), &http::Method::GET);
        assert_eq!(req.uri(), "https://api.trakt.tv/test");
        assert_eq!(
            req.headers().get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(req.headers().get("trakt-api-version").unwrap(), "2");
        assert_eq!(req.headers().get("trakt-api-key").unwrap(), "client id");
        assert_eq!(req.headers().get("Authorization").unwrap(), "Bearer token");
        assert_eq!(req.into_body(), "body");

        md.auth = AuthRequirement::Required;
        ctx.oauth_token = None;
        let result = construct_req(&ctx, &md, &(), &(), "body").unwrap_err();
        assert!(matches!(result, IntoHttpError::MissingToken));

        md.auth = AuthRequirement::Optional;
        ctx.oauth_token = None;

        let req = construct_req(&ctx, &md, &(), &(), "body").unwrap();
        assert_eq!(req.method(), &http::Method::GET);
        assert_eq!(req.uri(), "https://api.trakt.tv/test");
        assert_eq!(
            req.headers().get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(req.headers().get("trakt-api-version").unwrap(), "2");
        assert_eq!(req.headers().get("trakt-api-key").unwrap(), "client id");
        assert!(req.headers().get("Authorization").is_none());
        assert_eq!(req.into_body(), "body");

        md.auth = AuthRequirement::Optional;
        ctx.oauth_token = Some("token");

        let req = construct_req(&ctx, &md, &(), &(), "body").unwrap();
        assert_eq!(req.method(), &http::Method::GET);
        assert_eq!(req.uri(), "https://api.trakt.tv/test");
        assert_eq!(
            req.headers().get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(req.headers().get("trakt-api-version").unwrap(), "2");
        assert_eq!(req.headers().get("trakt-api-key").unwrap(), "client id");
        assert_eq!(req.headers().get("Authorization").unwrap(), "Bearer token");
        assert_eq!(req.into_body(), "body");
    }
}
