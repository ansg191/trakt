//! # A Pure Rust Library for the [Trakt.tv](https://trakt.tv) API
//!
//! [![Crates.io Version](https://img.shields.io/crates/v/trakt-rs)](https://crates.io/crates/trakt-rs)
//! [![docs.rs](https://img.shields.io/docsrs/trakt-rs)](https://docs.rs/trakt-rs)
//! ![Crates.io License](https://img.shields.io/crates/l/trakt-rs)
//! [![Rust](https://github.com/ansg191/trakt/actions/workflows/rust.yml/badge.svg)](https://github.com/ansg191/trakt/actions/workflows/rust.yml)
//! [![codecov](https://codecov.io/gh/ansg191/trakt/graph/badge.svg?token=5UO8NZJ2C1)](https://codecov.io/gh/ansg191/trakt)
//!
//! Trakt.tv API Documentation: [https://trakt.docs.apiary.io](https://trakt.docs.apiary.io)
//!
//! ## Usage
//!
//! This library does not provide a client for making HTTP(s) requests.
//! That is left to the user. This enables the user to use any HTTP client they prefer
//! (e.g. `reqwest`, `hyper`, `isahc`, etc.) with any TLS backend (e.g. `native-tls`, `rustls`, etc.)
//! in a synchronous or asynchronous manner.
//!
//! Instead, the library provides a set of request and response types that can be converted into the
//! general purpose [`http::Request`] and [`http::Response`] types.
//! The types fill out the entirety of the HTTP request, including the URL, headers, and body.
//! However, the user may still modify the request before sending it.
//!
//! The advantage of this approach is that the user has infinite flexibility in how they make requests.
//! They can use any HTTP client, any TLS backend, and any request/response handling mechanism.
//! Additionally, the user is free to make modifications to the request before sending it or the
//! response after receiving it.
//!
//! This also means this library has a smaller dependency tree, as it does not depend on
//! runtime or HTTP client libraries.
//!
//! ### Example
//!
//! ```no_run
//! use trakt_rs::{Request, Response};
//!
//! // Context required for all requests
//! let ctx = trakt_rs::Context {
//!     base_url: "https://api.trakt.tv",
//!     client_id: "client_id",
//!     oauth_token: None,
//! };
//!
//! // Create a request and convert it into an HTTP request
//! let req = trakt_rs::api::movies::summary::Request {
//!     id: trakt_rs::smo::Id::Imdb("tt123456".into()),
//! };
//! let http_req: http::Request<Vec<u8>> = req.try_into_http_request(ctx).unwrap();
//!
//! // Send the HTTP request using your preferred HTTP client
//! let response = http::Response::new(vec![]);
//!
//! // Convert the HTTP response into a Trakt response
//! let trakt_response = trakt_rs::api::movies::summary::Response::try_from_http_response(response).unwrap();
//!
//! println!("Movie: {:?}", trakt_response.0);
//! ```
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    clippy::as_underscore,
    clippy::clone_on_ref_ptr,
    clippy::format_push_string,
    clippy::mod_module_files,
    clippy::str_to_string
)]
#![allow(clippy::module_name_repetitions, clippy::redundant_pub_crate)]

pub mod api;
pub mod smo;
#[cfg(test)]
mod test;

pub use trakt_core::{
    error, AuthRequirement, Context, EmojiString, Metadata, PaginatedResponse, Pagination,
    PaginationResponse, Request, Response,
};

time::serde::format_description!(iso8601_date, Date, "[year]-[month]-[day]");

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use time::{Date, Month};

    use super::*;

    #[test]
    fn test_iso8601_date() {
        #[derive(Debug, Serialize, Deserialize)]
        struct TestDate {
            #[serde(with = "iso8601_date")]
            date: Date,
        }

        let date = TestDate {
            date: Date::from_calendar_date(2021, Month::December, 31).unwrap(),
        };
        let json = serde_json::to_string(&date).unwrap();
        assert_eq!(json, r#"{"date":"2021-12-31"}"#);

        let date = TestDate {
            date: Date::from_calendar_date(2024, Month::April, 1).unwrap(),
        };
        let json = serde_json::to_string(&date).unwrap();
        assert_eq!(json, r#"{"date":"2024-04-01"}"#);

        let json = r#"{"date":"2024-04-01"}"#;
        let date: TestDate = serde_json::from_str(json).unwrap();
        assert_eq!(
            date.date,
            Date::from_calendar_date(2024, Month::April, 1).unwrap()
        );
    }
}
