#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::as_underscore,
    clippy::clone_on_ref_ptr,
    clippy::format_push_string,
    clippy::mod_module_files,
    clippy::str_to_string
)]
#![allow(clippy::module_name_repetitions)]

use std::borrow::Cow;

use reqwest::header::{HeaderMap, HeaderValue};

pub mod api;
pub mod error;
mod request;
mod response;
pub mod smo;
mod utils;
mod url;

pub use request::*;
pub use response::*;
pub use utils::{Pagination, PaginationResponse};

const BASE_URL: &str = "https://api.trakt.tv";
const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub struct TraktApi {
    client: reqwest::Client,
    base_url: Cow<'static, str>,
}

impl TraktApi {
    /// Constructs a new instance of the `ApiClient` struct.
    ///
    /// This function takes a `client_id` parameter of type `HeaderValue` which represents the client ID
    /// used for authentication with the Trakt API.
    ///
    /// Returns a new `ApiClient` instance.
    ///
    /// # Panics
    /// This function will panic if the [`reqwest::Client`] fails to build.
    #[must_use]
    pub fn new(client_id: HeaderValue) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", HeaderValue::from_static("application/json"));
        headers.insert("trakt-api-version", HeaderValue::from_static("2"));
        headers.insert("trakt-api-key", client_id);

        let client = reqwest::Client::builder()
            .user_agent(USER_AGENT)
            .default_headers(headers)
            .build()
            .expect("Request client should not fail to build");

        Self {
            client,
            base_url: Cow::Borrowed(BASE_URL),
        }
    }

    pub async fn execute<R: Request>(&self, request: R) -> Result<R::Response, TraktError> {
        todo!()
        // let req: http::Request<Vec<u8>> = request
        //     .try_into_http_request(self.base_url.as_ref(), None)
        //     .map_err(TraktError::IntoHttp)?;
        // let req = req.try_into().unwrap();
        //
        // let mut response = self
        //     .client
        //     .execute(req)
        //     .await
        //     .map_err(TraktError::Reqwest)?;
        //
        // let mut builder = http::Response::builder()
        //     .status(response.status())
        //     .version(response.version());
        //
        // std::mem::swap(
        //     builder.headers_mut().expect("response should be valid"),
        //     response.headers_mut(),
        // );
        //
        // let res = builder
        //     .body(response.bytes().await.map_err(TraktError::Reqwest)?)
        //     .expect("response should be valid");
        //
        // R::Response::try_from_http_response(res).map_err(TraktError::FromHttp)
    }
}

#[derive(Debug)]
pub enum TraktError {
    Reqwest(reqwest::Error),
    IntoHttp(IntoHttpError),
    FromHttp(FromHttpError),
}
