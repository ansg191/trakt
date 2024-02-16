//! Show related endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/shows>

pub mod trending {
    //! Get trending shows
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/trending/get-trending-shows>
    use http::StatusCode;
    use serde::Deserialize;
    use trakt_core::{
        error::FromHttpError, handle_response_body, parse_from_header, Pagination,
        PaginationResponse,
    };

    use crate::smo::Show;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/trending",
    )]
    pub struct Request {
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
        pub trending_user_count: u64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
    pub struct ResponseItem {
        pub watchers: u64,
        pub show: Show,
    }

    impl trakt_core::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, FromHttpError> {
            let body = handle_response_body(&response, StatusCode::OK)?;
            let items = PaginationResponse::from_headers(body, response.headers())?;
            Ok(Self {
                items,
                trending_user_count: parse_from_header(
                    response.headers(),
                    "X-Trending-User-Count",
                )?,
            })
        }
    }
}

pub mod popular {
    //! Get popular shows
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/popular/get-popular-shows>

    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::Show;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/popular",
    )]
    pub struct Request {
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = OK)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub watchers: u64,
        pub show: Show,
    }
}

pub mod favorited {
    //! Get most favorited shows
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/favorited/get-the-most-favorited-shows>

    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::Show;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/favorited",
    )]
    pub struct Request {
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = OK)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub user_count: u64,
        pub show: Show,
    }
}

pub mod played {
    //! Get most played shows
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/favorited/get-the-most-played-shows>

    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Period, Show};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/played/{period}",
    )]
    pub struct Request {
        pub period: Period,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = OK)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub watcher_count: u64,
        pub play_count: u64,
        pub collected_count: u64,
        pub collector_count: u64,
        pub show: Show,
    }
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use isahc::ReadResponseExt;
    use serde_json::json;
    use trakt_core::{Context, PaginatedResponse, Request, Response};

    use super::*;

    #[test]
    fn test_trending() {
        let server = MockServer::start();

        let trending_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/shows/trending")
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", "abc")
                .query_param("page", "1")
                .query_param("limit", "10");
            then.status(200)
                .header("Content-Type", "application/json")
                .header("X-Trending-User-Count", "123")
                .header("X-Pagination-Page", "1")
                .header("X-Pagination-Limit", "10")
                .header("X-Pagination-Page-Count", "1")
                .header("X-Pagination-Item-Count", "1")
                .json_body(json!([
                    {
                        "watchers": 123,
                        "show": {
                            "title": "The Dark Knight",
                            "year": 2008,
                            "ids": {
                                "trakt": 16,
                                "slug": "the-dark-knight-2008",
                                "imdb": "tt0468569",
                                "tmdb": 155
                            }
                        }
                    }
                ]));
        });

        let ctx = Context {
            base_url: &server.base_url(),
            client_id: "abc",
            oauth_token: None,
        };

        let request = trending::Request::default();
        let http_req: http::Request<Vec<u8>> = request.try_into_http_request(ctx).unwrap();

        assert_eq!(
            http_req.uri(),
            &*format!("{}/shows/trending?page=1&limit=10", server.base_url())
        );
        assert_eq!(http_req.method(), http::Method::GET);
        assert_eq!(
            http_req.headers().get("Content-Type").unwrap(),
            "application/json"
        );
        assert_eq!(http_req.headers().get("trakt-api-key").unwrap(), "abc");
        assert_eq!(http_req.headers().get("trakt-api-version").unwrap(), "2");
        assert_eq!(http_req.headers().get("Authorization"), None);
        assert!(http_req.body().is_empty());

        let mut response = isahc::send(http_req).unwrap();
        let bytes = response.bytes().unwrap();
        let (parts, _) = response.into_parts();
        let response = http::Response::from_parts(parts, bytes);

        let response = trending::Response::try_from_http_response(response).unwrap();

        assert_eq!(response.items().len(), 1);
        assert_eq!(response.items()[0].watchers, 123);
        assert_eq!(response.items()[0].show.title, "The Dark Knight");
        assert_eq!(response.items()[0].show.year, 2008);
        assert_eq!(response.items()[0].show.ids.trakt, Some(16));

        assert_eq!(response.next_page(), None);

        trending_mock.assert();
    }
}
