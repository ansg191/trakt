use bytes::BufMut;
use http::{header::InvalidHeaderValue, Method};

use crate::response::Response;

/// Trait for requests.
///
/// All requests in the API should implement this trait.
/// It provides a method to convert the request into an HTTP request.
///
/// The implementing type can perform any necessary validation on the request before converting it
/// into an HTTP request.
///
/// The [`Self::Response`] associated type is the type that should be used to represent the response
/// returned by the server.
pub trait Request: Sized + Clone {
    type Response: Response;

    /// The metadata for the request.
    const METADATA: Metadata;

    /// Tries to convert the request into an HTTP request.
    ///
    /// On endpoints requiring authentication, the `token` field in `ctx` should be provided.
    /// If not, the request will fail to convert.
    ///
    /// # Arguments
    ///
    /// * `ctx`: The context for the request.
    ///
    /// # Errors
    /// This function will return an error if the request cannot be converted into an HTTP request.
    fn try_into_http_request<T: Default + BufMut>(
        self,
        ctx: Context,
    ) -> Result<http::Request<T>, IntoHttpError>;
}

/// Represents metadata for an API endpoint.
///
/// This struct holds information about the endpoint, such as the URL endpoint, HTTP method, and
/// authorization requirement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {
    /// The URL endpoint for the request.
    pub endpoint: &'static str,
    /// The HTTP method for the request.
    pub method: Method,
    /// Whether the request requires authorization.
    pub auth: bool,
}

/// Represents the universal context for an API request.
///
/// This struct contains the information needed to make an API request, such as the base URL,
/// client ID, and OAuth token if available.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Context<'a> {
    /// The base URL for the API.
    pub base_url: &'a str,
    /// The client ID for the API.
    pub client_id: &'a str,
    /// The OAuth token for the API, if requesting an authenticated endpoint.
    pub oauth_token: Option<&'a str>,
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
    UrlParams(#[from] crate::url::UrlError),
    #[error("Query params error: {0}")]
    QueryParams(#[from] serde_urlencoded::ser::Error),
}
