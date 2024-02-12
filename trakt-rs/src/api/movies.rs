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

pub mod updates_id {
    use time::OffsetDateTime;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/updates/{start_date}",
    )]
    pub struct Request {
        #[serde(with = "time::serde::iso8601")]
        pub start_date: OffsetDateTime,
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: crate::PaginationResponse<u32>,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body = crate::utils::handle_response_body(&response, http::StatusCode::OK)?;
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

pub mod aliases {
    use serde::Deserialize;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/aliases",
    )]
    pub struct Request {
        pub id: String,
    }

    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    pub struct ResponseItem {
        pub title: String,
        pub country: String,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body = crate::utils::handle_response_body(&response, http::StatusCode::OK)?;
            Ok(Self(body))
        }
    }
}

pub mod releases {
    use serde::Deserialize;

    use crate::smo::Country;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/releases/{country}",
    )]
    pub struct Request {
        pub id: String,
        pub country: Country,
    }

    #[derive(Debug)]
    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    pub struct ResponseItem {
        pub country: Country,
        pub certification: String,
        pub release_date: String,
        pub release_type: ReleaseType,
        pub note: Option<String>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum ReleaseType {
        Unknown,
        Premiere,
        Limited,
        Theatrical,
        Digital,
        Physical,
        TV,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body = crate::utils::handle_response_body(&response, http::StatusCode::OK)?;
            Ok(Self(body))
        }
    }
}

pub mod translations {
    use serde::Deserialize;

    use crate::smo::{Country, Language};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/translations/{language}",
    )]
    pub struct Request {
        pub id: String,
        pub language: Language,
    }

    #[derive(Debug)]
    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    pub struct ResponseItem {
        pub title: String,
        pub overview: String,
        pub tagline: String,
        pub language: Language,
        pub country: Country,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body = crate::utils::handle_response_body(&response, http::StatusCode::OK)?;
            Ok(Self(body))
        }
    }
}

pub mod comments {
    //! TODO: Implement
}

pub mod lists {
    //! TODO: Implement
    // use serde::Serialize;
    //
    // #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    // #[trakt(
    // response = Response,
    // endpoint = "/movies/{id}/lists/{tp}/{sort}",
    // )]
    // pub struct Request {
    //     pub id: String,
    //     pub tp: Type,
    //     pub sort: Sort,
    //     pub pagination: crate::utils::Pagination,
    // }
    //
    // #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize)]
    // #[serde(rename_all = "lowercase")]
    // pub enum Type {
    //     All,
    //     #[default]
    //     Personal,
    //     Official,
    //     Watchlist,
    //     Favorite,
    // }
    //
    // #[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize)]
    // #[serde(rename_all = "lowercase")]
    // pub enum Sort {
    //     #[default]
    //     Popular,
    //     Likes,
    //     Comments,
    //     Items,
    //     Added,
    //     Updated,
    // }
}

pub mod people {
    use serde::Deserialize;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/people",
    )]
    pub struct Request {
        pub id: String,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Response {
        pub cast: Vec<Character>,
        pub crew: Crew,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Character {
        pub characters: Vec<String>,
        pub person: crate::smo::Person,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct Crew {
        pub production: Vec<CrewMember>,
        pub art: Vec<CrewMember>,
        pub crew: Vec<CrewMember>,
        #[serde(rename = "costume & make-up")]
        pub costume_and_make_up: Vec<CrewMember>,
        pub directing: Vec<CrewMember>,
        pub writing: Vec<CrewMember>,
        pub sound: Vec<CrewMember>,
        pub camera: Vec<CrewMember>,
        #[serde(rename = "visual effects")]
        pub visual_effects: Vec<CrewMember>,
        pub lighting: Vec<CrewMember>,
        pub editing: Vec<CrewMember>,
    }

    #[derive(Debug, Clone, Deserialize)]
    pub struct CrewMember {
        pub jobs: Vec<String>,
        pub person: crate::smo::Person,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body = crate::utils::handle_response_body(&response, http::StatusCode::OK)?;
            Ok(body)
        }
    }
}

pub mod ratings {
    use serde::Deserialize;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/ratings",
    )]
    pub struct Request {
        pub id: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    pub struct Response {
        pub rating: f64,
        pub votes: u64,
        pub distribution: Distribution,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    pub struct Distribution {
        #[serde(rename = "1")]
        pub one: u32,
        #[serde(rename = "2")]
        pub two: u32,
        #[serde(rename = "3")]
        pub three: u32,
        #[serde(rename = "4")]
        pub four: u32,
        #[serde(rename = "5")]
        pub five: u32,
        #[serde(rename = "6")]
        pub six: u32,
        #[serde(rename = "7")]
        pub seven: u32,
        #[serde(rename = "8")]
        pub eight: u32,
        #[serde(rename = "9")]
        pub nine: u32,
        #[serde(rename = "10")]
        pub ten: u32,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            let body = crate::utils::handle_response_body(&response, http::StatusCode::OK)?;
            Ok(body)
        }
    }
}
