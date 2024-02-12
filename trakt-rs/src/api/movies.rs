//! Movie related endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/movies>

pub mod favorited {
    //! Get the most favorited movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/favorited>

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
    //! Get popular movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/popular/get-popular-movies>
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
    //! Get trending movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/trending/get-trending-movies>
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

pub mod played {
    //! Get the most played movies in a specific time period.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/played/get-the-most-played-movies>
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
    //! Get the most watched movies in a specific time period.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/watched/get-the-most-watched-movies>
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
    //! Get the most collected movies in a specific time period.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/collected/get-the-most-collected-movies>
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
    //! Get the most anticipated movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/anticipated/get-the-most-anticipated-movies>
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
    //! Get the top 10 box office movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/box-office/get-the-weekend-box-office>
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/boxoffice",
    )]
    pub struct Request;

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Response {
        pub movies: Vec<ResponseItem>,
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
            let movies = crate::utils::handle_response_body(&response, http::StatusCode::OK)?;
            Ok(Self { movies })
        }
    }
}

pub mod updates {
    //! Get all movies updated since a specific date.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/box-office/get-recently-updated-movies>
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
    //! Get recently update movie IDs since a specific date.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/updated-ids/get-recently-updated-movie-trakt-ids>

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
    //! Get a single movie's details.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/summary/get-a-movie>

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
    //! Get all title aliases for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/aliases/get-all-movie-aliases>

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
    //! Get all releases for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/releases/get-all-movie-releases>

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
    //! Get all translations for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/translations/get-all-movie-translations>

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
    //! Get all comments for a movie.
    //!
    //! If oauth is provided, comments from blocked users will be filtered out.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/comments/get-all-movie-comments>

    use serde::{Deserialize, Serialize};
    use time::OffsetDateTime;

    use crate::FromHttpError;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/comments/{sort}",
    auth = Optional,
    )]
    pub struct Request {
        pub id: String,
        pub sort: Sort,
        pub pagination: crate::utils::Pagination,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Sort {
        #[default]
        Newest,
        Oldest,
        Likes,
        Replies,
        Highest,
        Lowest,
        Plays,
    }

    #[derive(Debug, Clone, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: crate::PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    pub struct ResponseItem {
        pub id: u32,
        pub parent_id: Option<u32>,
        #[serde(with = "time::serde::iso8601")]
        pub created_at: OffsetDateTime,
        #[serde(with = "time::serde::iso8601")]
        pub updated_at: OffsetDateTime,
        pub comment: String,
        pub spoiler: bool,
        pub review: bool,
        pub replies: u32,
        pub likes: u32,
        pub user_stats: UserStats,
        pub user: crate::smo::User,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    pub struct UserStats {
        pub rating: u8,
        pub play_count: u32,
        pub completed_count: u32,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, FromHttpError> {
            let body = crate::utils::handle_response_body(&response, http::StatusCode::OK)?;
            let items = crate::PaginationResponse::from_headers(body, response.headers())?;
            Ok(Self { items })
        }
    }
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
    //! Get all people for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/lists/get-all-people-for-a-movie>

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
    //! Get rating distribution for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/ratings/get-movie-ratings>

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

pub mod related {
    //! Get related movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/related/get-related-movies>

    use crate::FromHttpError;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/related",
    )]
    pub struct Request {
        pub id: String,
    }

    #[derive(Debug, Clone, trakt_macros::Paginated)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: crate::PaginationResponse<crate::smo::Movie>,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, FromHttpError> {
            let body = crate::utils::handle_response_body(&response, http::StatusCode::OK)?;
            let items = crate::PaginationResponse::from_headers(body, response.headers())?;
            Ok(Self { items })
        }
    }
}

pub mod stats {
    //! Get stats for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/related/get-movie-stats>

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/stats",
    )]
    pub struct Request {
        pub id: String,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct Response {
        pub watchers: u32,
        pub plays: u32,
        pub collectors: u32,
        pub comments: u32,
        pub lists: u32,
        pub votes: u32,
        pub favorited: u32,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, crate::FromHttpError> {
            crate::utils::handle_response_body(&response, http::StatusCode::OK)
        }
    }
}

pub mod studio {
    //! TODO: Implement
}

pub mod watching {
    //! Get users currently watching a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/watching/get-users-currently-watching-a-movie>

    use crate::FromHttpError;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/watching",
    )]
    pub struct Request {
        pub id: String,
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Response {
        pub items: Vec<crate::smo::User>,
    }

    impl crate::Response for Response {
        fn try_from_http_response<T: AsRef<[u8]>>(
            response: http::Response<T>,
        ) -> Result<Self, FromHttpError> {
            Ok(Self {
                items: crate::utils::handle_response_body(&response, http::StatusCode::OK)?,
            })
        }
    }
}
