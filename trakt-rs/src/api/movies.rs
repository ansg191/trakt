//! Movie related endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/movies>

pub mod favorited {
    //! Get the most favorited movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/favorited>

    use serde::Deserialize;
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Movie, Period};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/favorited/{period}",
    )]
    pub struct Request {
        pub period: Period,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
    pub struct ResponseItem {
        pub user_count: usize,
        pub movie: Movie,
    }
}

pub mod popular {
    //! Get popular movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/popular/get-popular-movies>
    use trakt_core::PaginationResponse;

    use crate::smo::Movie;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/popular",
    )]
    pub struct Request {
        #[serde(flatten)]
        pub pagination: trakt_core::Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<Movie>,
    }
}

pub mod trending {
    //! Get trending movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/trending/get-trending-movies>
    use http::StatusCode;
    use serde::Deserialize;
    use trakt_core::{
        error::FromHttpError, handle_response_body, parse_from_header, Pagination,
        PaginationResponse,
    };

    use crate::smo::Movie;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/trending",
    )]
    pub struct Request {
        #[serde(flatten)]
        pub pagination: Pagination,
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

    impl trakt_core::Response for Response {
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
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Movie, Period};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/played/{period}",
    )]
    pub struct Request {
        pub period: Period,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub watcher_count: usize,
        pub play_count: usize,
        pub collected_count: usize,
        pub movie: Movie,
    }
}

pub mod watched {
    //! Get the most watched movies in a specific time period.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/watched/get-the-most-watched-movies>
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Movie, Period};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/watched/{period}",
    )]
    pub struct Request {
        pub period: Period,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub watcher_count: usize,
        pub play_count: usize,
        pub collected_count: usize,
        pub movie: Movie,
    }
}

pub mod collected {
    //! Get the most collected movies in a specific time period.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/collected/get-the-most-collected-movies>
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Movie, Period};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/collected/{period}",
    )]
    pub struct Request {
        pub period: Period,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub watcher_count: usize,
        pub play_count: usize,
        pub collected_count: usize,
        pub movie: Movie,
    }
}

pub mod anticipated {
    //! Get the most anticipated movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/anticipated/get-the-most-anticipated-movies>
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::Movie;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/anticipated",
    )]
    pub struct Request {
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub list_count: usize,
        pub movie: Movie,
    }
}

pub mod boxoffice {
    //! Get the top 10 box office movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/box-office/get-the-weekend-box-office>

    use crate::smo::Movie;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/boxoffice",
    )]
    pub struct Request;

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Response)]
    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct ResponseItem {
        pub revenue: usize,
        pub movie: Movie,
    }
}

pub mod updates {
    //! Get all movies updated since a specific date.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/box-office/get-recently-updated-movies>
    use time::OffsetDateTime;
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::Movie;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/updates/{start_date}",
    )]
    pub struct Request {
        #[serde(with = "time::serde::iso8601")]
        pub start_date: OffsetDateTime,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Response)]
    #[trakt(expected = OK)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<Movie>,
    }
}

pub mod updates_id {
    //! Get recently update movie IDs since a specific date.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/updated-ids/get-recently-updated-movie-trakt-ids>

    use time::OffsetDateTime;
    use trakt_core::{Pagination, PaginationResponse};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/updates/id/{start_date}",
    )]
    pub struct Request {
        #[serde(with = "time::serde::iso8601")]
        pub start_date: OffsetDateTime,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Response)]
    #[trakt(expected = OK)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<u32>,
    }
}

pub mod summary {
    //! Get a single movie's details.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/summary/get-a-movie>

    use crate::smo::{Id, Movie};

    #[derive(Debug, Clone, Eq, PartialEq, trakt_macros::Request)]
    #[trakt(response = Response, endpoint = "/movies/{id}")]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Movie);
}

pub mod aliases {
    //! Get all title aliases for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/aliases/get-all-movie-aliases>

    use serde::Deserialize;

    use crate::smo::Id;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/aliases",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
    pub struct ResponseItem {
        pub title: String,
        pub country: String,
    }
}

pub mod releases {
    //! Get all releases for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/releases/get-all-movie-releases>

    use serde::Deserialize;

    use crate::smo::{Country, Id};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/releases/{country}",
    )]
    pub struct Request {
        pub id: Id,
        pub country: Country,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
    pub struct ResponseItem {
        pub country: Country,
        pub certification: String,
        pub release_date: String,
        pub release_type: ReleaseType,
        pub note: Option<String>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
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
}

pub mod translations {
    //! Get all translations for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/translations/get-all-movie-translations>

    use serde::Deserialize;

    use crate::smo::{Country, Id, Language};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/translations/{language}",
    )]
    pub struct Request {
        pub id: Id,
        pub language: Language,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
    pub struct ResponseItem {
        pub title: String,
        pub overview: String,
        pub tagline: String,
        pub language: Language,
        pub country: Country,
    }
}

pub mod comments {
    //! Get all comments for a movie.
    //!
    //! If oauth is provided, comments from blocked users will be filtered out.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/comments/get-all-movie-comments>
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Comment, Sort};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/comments/{sort}",
    auth = Optional,
    )]
    pub struct Request {
        pub id: String,
        pub sort: Sort,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = OK)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<Comment>,
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
    //     pub pagination: Pagination,
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

    use crate::smo::{Id, Person};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/people",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub cast: Vec<Character>,
        pub crew: Crew,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
    pub struct Character {
        pub characters: Vec<String>,
        pub person: Person,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
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

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
    pub struct CrewMember {
        pub jobs: Vec<String>,
        pub person: Person,
    }
}

pub mod ratings {
    //! Get rating distribution for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/ratings/get-movie-ratings>

    use serde::Deserialize;

    use crate::smo::{Id, Ratings};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/ratings",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, trakt_macros::Response)]
    pub struct Response(pub Ratings);
}

pub mod related {
    //! Get related movies.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/related/get-related-movies>

    use trakt_core::PaginationResponse;

    use crate::smo::{Id, Movie};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/related",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = OK)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<Movie>,
    }
}

pub mod stats {
    //! Get stats for a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/related/get-movie-stats>
    use crate::smo::Id;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/stats",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub watchers: u32,
        pub plays: u32,
        pub collectors: u32,
        pub comments: u32,
        pub lists: u32,
        pub votes: u32,
        pub favorited: u32,
    }
}

pub mod studio {
    //! Get movie studios
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/studios/get-movie-studios>

    use crate::smo::{Id, Studio};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/studios",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<Studio>);
}

pub mod watching {
    //! Get users currently watching a movie.
    //!
    //! <https://trakt.docs.apiary.io/#reference/movies/watching/get-users-currently-watching-a-movie>
    use crate::smo::{Id, User};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/movies/{id}/watching",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<User>);
}

#[cfg(test)]
mod tests {
    use httpmock::prelude::*;
    use serde_json::json;
    use trakt_core::{Context, PaginatedResponse, Request};

    use super::*;

    #[test]
    pub fn test_popular() {
        let server = MockServer::start();

        let popular_mock = server.mock(|when, then| {
            when.method(GET)
                .path("/movies/popular")
                .header("Content-Type", "application/json")
                .header("trakt-api-key", "abc")
                .header("trakt-api-version", "2")
                .query_param("page", "1")
                .query_param("limit", "10");
            then.status(200)
                .header("Content-Type", "application/json")
                .header("X-Pagination-Page", "1")
                .header("X-Pagination-Limit", "10")
                .header("X-Pagination-Page-Count", "1")
                .header("X-Pagination-Item-Count", "2")
                .json_body(json!([
                    {
                        "title": "The Dark Knight",
                        "year": 2008,
                        "ids": {
                            "trakt": 16,
                            "slug": "the-dark-knight-2008",
                            "imdb": "tt0468569",
                            "tmdb": 155
                        }
                    },
                    {
                        "title": "Fight Club",
                        "year": 1999,
                        "ids": {
                            "trakt": 727,
                            "slug": "fight-club-1999",
                            "imdb": "tt0137523",
                            "tmdb": 550
                        }
                    }
                ]));
        });

        let ctx = Context {
            base_url: &server.base_url(),
            client_id: "abc",
            oauth_token: None,
        };

        let request = popular::Request::default();
        let http_req: http::Request<Vec<u8>> = request.try_into_http_request(ctx).unwrap();

        assert_eq!(
            http_req.uri(),
            &*format!("{}/movies/popular?page=1&limit=10", server.base_url())
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

        let response = crate::test::execute(ctx, request).unwrap();

        assert_eq!(response.items().len(), 2);
        assert_eq!(response.items()[0].title, "The Dark Knight");
        assert_eq!(response.items()[0].year, 2008);
        assert_eq!(response.items()[0].ids.trakt, Some(16));
        assert_eq!(response.items()[1].title, "Fight Club");
        assert_eq!(response.items()[1].year, 1999);
        assert_eq!(response.items()[1].ids.trakt, Some(727));

        assert_eq!(response.next_page(), None);

        popular_mock.assert();
    }
}
