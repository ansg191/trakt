//! Scrobble endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/scrobble>

use serde::Deserialize;

use crate::smo::{Episode, Ids, Movie, Sharing, Show};

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "lowercase")]
enum BodyInner {
    Movie { ids: Ids },
    Episode { ids: Ids },
}

#[derive(Debug, serde::Serialize)]
struct Body {
    #[serde(flatten)]
    inner: BodyInner,
    progress: f64,
}

mod _private {
    use crate::smo::{Episode, Movie};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum CheckinItemType {
        Movie,
        Episode,
    }

    pub trait Sealed {
        const KEY: CheckinItemType;
    }

    impl Sealed for Movie {
        const KEY: CheckinItemType = CheckinItemType::Movie;
    }

    impl Sealed for Episode {
        const KEY: CheckinItemType = CheckinItemType::Episode;
    }
}

pub trait ScrobbleItem: _private::Sealed + Clone {
    type Response: trakt_core::Response;
}

impl ScrobbleItem for Movie {
    type Response = MovieResponse;
}

impl ScrobbleItem for Episode {
    type Response = EpisodeResponse;
}

#[derive(Debug, Clone, PartialEq, Deserialize, trakt_macros::Response)]
#[trakt(expected = CREATED)]
pub struct MovieResponse {
    pub id: u64,
    pub action: Action,
    pub progress: f64,
    pub sharing: Sharing,
    pub movie: Movie,
}

#[derive(Debug, Clone, PartialEq, Deserialize, trakt_macros::Response)]
#[trakt(expected = CREATED)]
pub struct EpisodeResponse {
    pub id: u64,
    pub action: Action,
    pub progress: f64,
    pub sharing: Sharing,
    pub episode: Episode,
    pub show: Show,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
    Start,
    Pause,
    Scrobble,
}

pub mod start {
    //! Start watching in media center
    //!
    //! <https://trakt.docs.apiary.io/#reference/scrobble/start/start-watching-in-a-media-center>

    use bytes::BufMut;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    use super::{Body, BodyInner, ScrobbleItem, _private::CheckinItemType};
    use crate::smo::{Episode, Id, Ids, Movie};

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request<I: ScrobbleItem> {
        pub id: Id,
        pub progress: f64,
        _phantom: std::marker::PhantomData<I>,
    }

    impl<I: ScrobbleItem> Request<I> {
        #[must_use]
        #[inline]
        pub const fn new(id: Id, progress: f64) -> Self {
            Self {
                id,
                progress,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl Request<Movie> {
        #[must_use]
        #[inline]
        pub const fn new_movie(id: Id, progress: f64) -> Self {
            Self::new(id, progress)
        }
    }

    impl Request<Episode> {
        #[must_use]
        #[inline]
        pub const fn new_episode(id: Id, progress: f64) -> Self {
            Self::new(id, progress)
        }
    }

    impl<I: ScrobbleItem> trakt_core::Request for Request<I> {
        type Response = I::Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/scrobble/start",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::Required,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let body = T::default();
            let mut writer = body.writer();

            let json = Body {
                inner: match I::KEY {
                    CheckinItemType::Movie => BodyInner::Movie {
                        ids: Ids::from(self.id),
                    },
                    CheckinItemType::Episode => BodyInner::Episode {
                        ids: Ids::from(self.id),
                    },
                },
                progress: self.progress,
            };

            serde_json::to_writer(&mut writer, &json)?;

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }
}

pub mod pause {
    //! Pause watching in media center
    //!
    //! <https://trakt.docs.apiary.io/#reference/scrobble/pause/pause-watching-in-a-media-center>

    use bytes::BufMut;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    use super::{Body, BodyInner, _private::CheckinItemType};
    use crate::{
        api::scrobble::ScrobbleItem,
        smo::{Episode, Id, Ids, Movie},
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request<I: ScrobbleItem> {
        pub id: Id,
        pub progress: f64,
        _phantom: std::marker::PhantomData<I>,
    }

    impl<I: ScrobbleItem> Request<I> {
        #[must_use]
        #[inline]
        pub const fn new(id: Id, progress: f64) -> Self {
            Self {
                id,
                progress,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl Request<Movie> {
        #[must_use]
        #[inline]
        pub const fn new_movie(id: Id, progress: f64) -> Self {
            Self::new(id, progress)
        }
    }

    impl Request<Episode> {
        #[must_use]
        #[inline]
        pub const fn new_episode(id: Id, progress: f64) -> Self {
            Self::new(id, progress)
        }
    }

    impl<I: ScrobbleItem> trakt_core::Request for Request<I> {
        type Response = I::Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/scrobble/pause",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::Required,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let body = T::default();
            let mut writer = body.writer();

            let json = Body {
                inner: match I::KEY {
                    CheckinItemType::Movie => BodyInner::Movie {
                        ids: Ids::from(self.id),
                    },
                    CheckinItemType::Episode => BodyInner::Episode {
                        ids: Ids::from(self.id),
                    },
                },
                progress: self.progress,
            };
            serde_json::to_writer(&mut writer, &json)?;

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }
}

pub mod stop {
    //! Stop watching in media center
    //!
    //! <https://trakt.docs.apiary.io/#reference/scrobble/stop/stop-or-finish-watching-in-a-media-center>

    use bytes::BufMut;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    use super::{Body, BodyInner, _private::CheckinItemType};
    use crate::{
        api::scrobble::ScrobbleItem,
        smo::{Episode, Id, Ids, Movie},
    };

    #[derive(Debug, Clone, PartialEq)]
    pub struct Request<I: ScrobbleItem> {
        pub id: Id,
        pub progress: f64,
        _phantom: std::marker::PhantomData<I>,
    }

    impl<I: ScrobbleItem> Request<I> {
        #[must_use]
        #[inline]
        pub const fn new(id: Id, progress: f64) -> Self {
            Self {
                id,
                progress,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl Request<Movie> {
        #[must_use]
        #[inline]
        pub const fn new_movie(id: Id, progress: f64) -> Self {
            Self::new(id, progress)
        }
    }

    impl Request<Episode> {
        #[must_use]
        #[inline]
        pub const fn new_episode(id: Id, progress: f64) -> Self {
            Self::new(id, progress)
        }
    }

    impl<I: ScrobbleItem> trakt_core::Request for Request<I> {
        type Response = I::Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/scrobble/stop",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::Required,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let body = T::default();
            let mut writer = body.writer();

            let json = Body {
                inner: match I::KEY {
                    CheckinItemType::Movie => BodyInner::Movie {
                        ids: Ids::from(self.id),
                    },
                    CheckinItemType::Episode => BodyInner::Episode {
                        ids: Ids::from(self.id),
                    },
                },
                progress: self.progress,
            };
            serde_json::to_writer(&mut writer, &json)?;

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use serde_json::json;
    use trakt_core::Context;

    use super::*;
    use crate::{smo::Id, test::assert_req};

    const CTX: Context = Context {
        base_url: "https://api.trakt.tv",
        client_id: "client id",
        oauth_token: Some("token"),
    };

    #[test]
    fn test_start() {
        let exp = json!({
            "movie": { "ids": { "trakt": 1 } },
            "progress": 0.0
        });
        let req = start::Request::new_movie(Id::Trakt(1), 0.0);
        assert_req!(CTX, req, "https://api.trakt.tv/scrobble/start", &exp);

        let exp = json!({
            "episode": { "ids": { "slug": "abc" } },
            "progress": 5.0
        });
        let req = start::Request::new_episode(Id::Slug("abc".into()), 5.0);
        assert_req!(CTX, req, "https://api.trakt.tv/scrobble/start", &exp);
    }

    #[test]
    fn test_pause() {
        let exp = json!({
            "movie": { "ids": { "tvdb": 1 } },
            "progress": 0.0
        });
        let req = pause::Request::new_movie(Id::Tvdb(1), 0.0);
        assert_req!(CTX, req, "https://api.trakt.tv/scrobble/pause", &exp);

        let exp = json!({
            "episode": { "ids": { "imdb": "tt12345" } },
            "progress": 10.0
        });
        let req = pause::Request::new_episode(Id::Imdb("tt12345".into()), 10.0);
        assert_req!(CTX, req, "https://api.trakt.tv/scrobble/pause", &exp);
    }

    #[test]
    fn test_stop() {
        let exp = json!({
            "movie": { "ids": { "tmdb": 1 } },
            "progress": 0.0
        });
        let req = stop::Request::new_movie(Id::Tmdb(1), 0.0);
        assert_req!(CTX, req, "https://api.trakt.tv/scrobble/stop", &exp);

        let exp = json!({
            "episode": { "ids": { "slug": "abc" } },
            "progress": 50.0
        });
        let req = stop::Request::new_episode(Id::Slug("abc".into()), 50.0);
        assert_req!(CTX, req, "https://api.trakt.tv/scrobble/stop", &exp);
    }
}
