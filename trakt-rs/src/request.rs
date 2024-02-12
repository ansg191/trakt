use bytes::BufMut;
use http::{header::InvalidHeaderValue, Method};

use crate::response::Response;

pub trait Request: Sized + Clone {
    type Response: Response;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Metadata {
    pub endpoint: &'static str,
    pub method: Method,
    pub auth: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Context<'a> {
    pub base_url: &'a str,
    pub client_id: &'a str,
    pub oauth_token: Option<&'a str>,
}

#[derive(Debug, thiserror::Error)]
pub enum IntoHttpError {
    #[error("JSON Error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Invalid Header Value: {0}")]
    Header(#[from] InvalidHeaderValue),
    #[error("HTTP Error: {0}")]
    Http(#[from] http::Error),
    #[error("Url params error: {0}")]
    UrlParams(#[from] crate::url::Error),
    #[error("Query params error: {0}")]
    QueryParams(#[from] serde_urlencoded::ser::Error),
}
