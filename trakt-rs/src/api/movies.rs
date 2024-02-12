pub mod favorited {
    use http::StatusCode;
    use serde::Deserialize;

    use crate::{smo::Movie, utils::handle_response_body, FromHttpError, PaginationResponse};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/favorited/{period}",
    )]
    pub struct Request {
        pub period: crate::smo::Period,
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    pub struct ResponseItem {
        pub user_count: usize,
        pub movie: Movie,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, FromHttpError> {
            let body: Vec<ResponseItem> = handle_response_body(&response, StatusCode::OK)?;

            let items = PaginationResponse::from_headers(body, response.headers())?;

            Ok(Self { items })
        }
    }
}

pub mod popular {
    use http::StatusCode;

    use crate::{smo::Movie, utils::handle_response_body, FromHttpError, PaginationResponse};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/popular",
    )]
    pub struct Request {
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<Movie>,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, FromHttpError> {
            let body: Vec<Movie> = handle_response_body(&response, StatusCode::OK)?;

            let items = PaginationResponse::from_headers(body, response.headers())?;

            Ok(Self { items })
        }
    }
}

pub mod trending {
    use http::StatusCode;
    use serde::Deserialize;

    use crate::{
        smo::Movie,
        utils::{handle_response_body, parse_from_header},
        FromHttpError, PaginationResponse,
    };

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/trending",
    )]
    pub struct Request {
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
        pub trending_user_count: usize,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    pub struct ResponseItem {
        pub watchers: usize,
        pub movie: Movie,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, FromHttpError> {
            let body: Vec<ResponseItem> = handle_response_body(&response, StatusCode::OK)?;

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

pub mod playing {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/played/{period}",
    )]
    pub struct Request {
        pub period: crate::smo::Period,
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: crate::PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub watcher_count: usize,
        pub play_count: usize,
        pub collected_count: usize,
        pub movie: crate::smo::Movie,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body: Vec<ResponseItem> =
                crate::utils::handle_response_body(&response, http::StatusCode::OK)?;

            let items = crate::PaginationResponse::from_headers(body, response.headers())?;

            Ok(Self { items })
        }
    }
}

pub mod watched {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/watched/{period}",
    )]
    pub struct Request {
        pub period: crate::smo::Period,
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: crate::PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub watcher_count: usize,
        pub play_count: usize,
        pub collected_count: usize,
        pub movie: crate::smo::Movie,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body: Vec<ResponseItem> =
                crate::utils::handle_response_body(&response, http::StatusCode::OK)?;

            let items = crate::PaginationResponse::from_headers(body, response.headers())?;

            Ok(Self { items })
        }
    }
}

pub mod collected {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/collected/{period}",
    )]
    pub struct Request {
        pub period: crate::smo::Period,
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: crate::PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub watcher_count: usize,
        pub play_count: usize,
        pub collected_count: usize,
        pub movie: crate::smo::Movie,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body: Vec<ResponseItem> =
                crate::utils::handle_response_body(&response, http::StatusCode::OK)?;

            let items = crate::PaginationResponse::from_headers(body, response.headers())?;

            Ok(Self { items })
        }
    }
}

pub mod anticipated {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/anticipated",
    )]
    pub struct Request {
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: crate::PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub list_count: usize,
        pub movie: crate::smo::Movie,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body: Vec<ResponseItem> =
                crate::utils::handle_response_body(&response, http::StatusCode::OK)?;

            let items = crate::PaginationResponse::from_headers(body, response.headers())?;

            Ok(Self { items })
        }
    }
}

pub mod boxoffice {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/boxoffice",
    )]
    pub struct Request {
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: crate::PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub revenue: usize,
        pub movie: crate::smo::Movie,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body: Vec<ResponseItem> =
                crate::utils::handle_response_body(&response, http::StatusCode::OK)?;

            let items = crate::PaginationResponse::from_headers(body, response.headers())?;

            Ok(Self { items })
        }
    }
}

pub mod updates {
    use time::OffsetDateTime;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/updates/{start_date}",
    )]
    pub struct Request {
        #[serde(with = "time::serde::iso8601")]
        pub start_date: OffsetDateTime,
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: crate::PaginationResponse<crate::smo::Movie>,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body: Vec<crate::smo::Movie> =
                crate::utils::handle_response_body(&response, http::StatusCode::OK)?;

            let items = crate::PaginationResponse::from_headers(body, response.headers())?;

            Ok(Self { items })
        }
    }

}

pub mod summary {
    use http::StatusCode;

    use crate::utils::handle_response_body;

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Request)]
    #[trakt(response = Response, endpoint = "/movies/{id}")]
    pub struct Request {
        pub id: String,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Response {
        pub item: crate::smo::Movie,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            Ok(Self {
                item: handle_response_body(&response, StatusCode::OK)?,
            })
        }
    }
}
