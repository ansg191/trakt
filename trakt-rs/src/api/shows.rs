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

pub mod watched {
    //! Get most watched shows
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/watched/get-the-most-watched-shows>
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Period, Show};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/watched/{period}",
    )]
    pub struct Request {
        pub period: Period,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
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

pub mod collected {
    //! Get most collected shows
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/watched/get-the-most-collected-shows>
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Period, Show};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/collected",
    )]
    pub struct Request {
        pub period: Period,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub watcher_count: u64,
        pub play_count: u64,
        pub collector_count: u64,
        pub collected_count: u64,
        pub show: Show,
    }
}

pub mod anticipated {
    //! Get most anticipated shows
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/anticipated/get-the-most-anticipated-shows>
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::Show;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/anticipated",
    )]
    pub struct Request {
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub list_count: u64,
        pub show: Show,
    }
}

pub mod updates {
    //! Get all shows updated since a specific date
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/anticipated/get-recently-updated-shows>

    use time::OffsetDateTime;
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::Show;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/updates/{start_date}",
    )]
    pub struct Request {
        #[serde(with = "time::serde::iso8601")]
        pub start_date: OffsetDateTime,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub updated_at: OffsetDateTime,
        pub show: Show,
    }
}

pub mod updates_id {
    //! Get recently updated show IDs since a specific date
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/updates/get-recently-updated-show-trakt-ids>

    use time::OffsetDateTime;
    use trakt_core::Pagination;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/updates/id/{start_date}",
    )]
    pub struct Request {
        #[serde(with = "time::serde::iso8601")]
        pub start_date: OffsetDateTime,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<u64>);
}

pub mod summary {
    //! Get a single show
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/summary/get-a-single-show>

    use crate::smo::{Id, Show};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Show);
}

pub mod aliases {
    //! Gets all title aliases for a show
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/aliases/get-all-show-aliases>

    use crate::smo::{Country, Id};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/aliases",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub title: String,
        pub country: Country,
    }
}

pub mod certifications {
    //! Gets all content certifications for a show
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/certifications/get-all-show-certifications>

    use crate::smo::{Country, Id};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/certifications",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub certification: String,
        pub country: Country,
    }
}

pub mod translation {
    //! Gets all show translations
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/certifications/get-all-show-translations>

    use crate::smo::{Country, Id, Language};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/translations/{language}",
    )]
    pub struct Request {
        pub id: Id,
        pub language: Language,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<ResponseItem>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        pub title: String,
        pub overview: String,
        pub tagline: Option<String>,
        pub language: Language,
        pub country: Country,
    }
}

pub mod comments {
    //! Get all top level comments for a show
    //!
    //! If oauth is provided, comments from blocked users will be filtered out.
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/translations/get-all-show-comments>

    use trakt_core::PaginationResponse;

    use crate::smo::{Comment, Id, Sort};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/comments/{sort}",
    auth = Optional,
    )]
    pub struct Request {
        id: Id,
        sort: Sort,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub comments: PaginationResponse<Comment>,
    }
}

pub mod lists {
    //! Get all lists that contain this show
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/lists/get-lists-containing-this-show>

    use serde::Serialize;
    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Id, List};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/lists/{tp}/{sort}"
    )]
    pub struct Request {
        pub id: Id,
        pub tp: Option<Type>,
        pub sort: Option<Sort>,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize)]
    pub enum Type {
        #[default]
        All,
        Personal,
        Official,
        Watchlist,
        Favorites,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Sort {
        #[default]
        Popular,
        Likes,
        Comments,
        Items,
        Added,
        Updated,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub lists: PaginationResponse<List>,
    }
}

pub mod collection_progress {
    //! Get show collection progress
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/collection-progress/get-show-collection-progress>

    use crate::smo::{Episode, Id, Season};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/progress/collection",
    auth = Required,
    )]
    pub struct Request {
        pub id: Id,
        pub hidden: bool,
        pub specials: bool,
        pub count_specials: bool,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub aired: u64,
        pub completed: u64,
        #[serde(with = "time::serde::iso8601::option")]
        pub last_collected_at: Option<time::OffsetDateTime>,
        pub seasons: Vec<SeasonCollection>,
        pub hidden_seasons: Vec<Season>,
        pub next_episode: Option<Episode>,
        pub last_episode: Option<Episode>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct SeasonCollection {
        pub number: u64,
        pub title: String,
        pub aired: u64,
        pub completed: u64,
        pub episodes: Vec<EpisodeCollection>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct EpisodeCollection {
        pub number: u64,
        pub completed: bool,
        #[serde(with = "time::serde::iso8601::option")]
        pub collected_at: Option<time::OffsetDateTime>,
    }
}

pub mod watched_progress {
    //! Get show watched progress
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/collection-progress/get-show-watched-progress>

    use crate::smo::{Episode, Id, Season};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/progress/watched",
    auth = Required,
    )]
    pub struct Request {
        pub id: Id,
        pub hidden: bool,
        pub specials: bool,
        pub count_specials: bool,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub aired: u64,
        pub completed: u64,
        #[serde(with = "time::serde::iso8601::option")]
        pub last_watched_at: Option<time::OffsetDateTime>,
        pub seasons: Vec<SeasonWatched>,
        pub hidden_seasons: Vec<Season>,
        pub next_episode: Option<Episode>,
        pub last_episode: Option<Episode>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct SeasonWatched {
        pub number: u64,
        pub title: String,
        pub aired: u64,
        pub completed: u64,
        pub episodes: Vec<EpisodeWatched>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct EpisodeWatched {
        pub number: u64,
        pub completed: bool,
        #[serde(with = "time::serde::iso8601::option")]
        pub last_watched_at: Option<time::OffsetDateTime>,
    }
}

pub mod reset {
    //! Resetting show progress
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/reset-watched-progress>

    #[allow(clippy::module_inception)]
    pub mod reset {
        //! Reset show progress
        //!
        //! <https://trakt.docs.apiary.io/#reference/shows/reset-watched-progress/reset-show-progress>

        use crate::smo::Id;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/shows/{id}/progress/watched/reset",
        method = POST,
        auth = Required,
        )]
        pub struct Request {
            pub id: Id,
        }

        #[derive(
            Debug, Copy, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response,
        )]
        pub struct Response {
            #[serde(with = "time::serde::iso8601")]
            pub reset_at: time::OffsetDateTime,
        }
    }

    pub mod undo {
        //! Undo show progress reset
        //!
        //! <https://trakt.docs.apiary.io/#reference/shows/reset-watched-progress/undo-reset-show-progress>

        use crate::smo::Id;

        #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
        #[trakt(
        response = Response,
        endpoint = "/shows/{id}/progress/watched/reset",
        method = DELETE,
        auth = Required,
        )]
        pub struct Request {
            pub id: Id,
        }

        #[derive(
            Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, trakt_macros::Response,
        )]
        #[trakt(expected = NO_CONTENT)]
        pub struct Response;
    }
}

pub mod people {
    //! TODO: Implement
}

pub mod ratings {
    //! Get show ratings
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/ratings/get-show-ratings>

    use crate::smo::{Id, Ratings};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/ratings",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, PartialEq, serde::Deserialize, trakt_macros::Response)]
    pub struct Response(pub Ratings);
}

pub mod related {
    //! Get related shows
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/ratings/get-related-shows>

    use trakt_core::{Pagination, PaginationResponse};

    use crate::smo::{Id, Show};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/related",
    )]
    pub struct Request {
        pub id: Id,
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<Show>,
    }
}

pub mod stats {
    //! Get show stats
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/related/get-show-stats>

    use crate::smo::Id;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/stats",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub watchers: u64,
        pub plays: u64,
        pub collectors: u64,
        pub collected_episodes: u64,
        pub comments: u64,
        pub lists: u64,
        pub votes: u64,
        pub favorited: u64,
    }
}

pub mod studio {
    //! Get show studios
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/stats/get-show-studios>

    use crate::smo::{Id, Studio};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/studios",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<Studio>);
}

pub mod watching {
    //! Get users watching a show right now
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/watching/get-users-watching-right-now>

    use crate::smo::{Id, User};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/watching",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<User>);
}

pub mod next_episode {
    //! Get next scheduled to air episode
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/next-episode/get-next-episode>

    use crate::smo::{Episode, Id};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/next_episode",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Episode);
}

pub mod last_episode {
    //! Gets the most recently aired episode
    //!
    //! <https://trakt.docs.apiary.io/#reference/shows/last-episode/get-last-episode>

    use crate::smo::{Episode, Id};

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/shows/{id}/last_episode",
    )]
    pub struct Request {
        pub id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Episode);
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
