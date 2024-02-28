//! Checkin
//!
//! <https://trakt.docs.apiary.io/#reference/checkin>

pub mod checkin {
    #![allow(clippy::module_inception)]
    //! Check into an item
    //!
    //! <https://trakt.docs.apiary.io/#reference/checkin/checkin/check-into-an-item>

    use bytes::BufMut;
    use serde::Deserialize;
    use serde_json::{json, Value};
    use time::OffsetDateTime;
    use trakt_core::{error::IntoHttpError, AuthRequirement, Context, Metadata};

    use crate::smo::{Episode, Id, Ids, Movie, Sharing, Show};

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Request<I: CheckinItem> {
        pub id: Id,
        pub sharing: Option<Sharing>,
        pub message: Option<String>,
        _phantom: std::marker::PhantomData<I>,
    }

    impl<I: CheckinItem> Request<I> {
        #[must_use]
        #[inline]
        pub const fn new(id: Id) -> Self {
            Self {
                id,
                sharing: None,
                message: None,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl Request<Movie> {
        #[must_use]
        #[inline]
        pub const fn new_movie(id: Id) -> Self {
            Self::new(id)
        }
    }

    impl Request<Episode> {
        #[must_use]
        #[inline]
        pub const fn new_episode(id: Id) -> Self {
            Self::new(id)
        }
    }

    impl<I: Clone + CheckinItem> trakt_core::Request for Request<I> {
        type Response = I::Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/checkin",
            method: http::Method::POST,
            auth: AuthRequirement::Required,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let body = T::default();
            let mut writer = body.writer();

            let json = Value::Object({
                let mut map = serde_json::Map::new();
                map.insert(I::KEY.to_owned(), json!({ "ids": Ids::from(self.id) }));
                if let Some(sharing) = self.sharing {
                    map.insert("sharing".to_owned(), json!(sharing));
                }
                if let Some(message) = self.message {
                    map.insert("message".to_owned(), json!(message));
                }
                map
            });

            serde_json::to_writer(&mut writer, &json)?;

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }

    mod _private {
        use crate::smo::{Episode, Movie};

        pub trait Sealed {
            const KEY: &'static str;
        }

        impl Sealed for Movie {
            const KEY: &'static str = "movie";
        }

        impl Sealed for Episode {
            const KEY: &'static str = "episode";
        }
    }

    pub trait CheckinItem: _private::Sealed {
        type Response: trakt_core::Response;
    }

    impl CheckinItem for Movie {
        type Response = MovieResponse;
    }

    impl CheckinItem for Episode {
        type Response = EpisodeResponse;
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, trakt_macros::Response)]
    #[trakt(expected = CREATED)]
    pub struct MovieResponse {
        pub id: u64,
        #[serde(with = "time::serde::iso8601")]
        pub watched_at: OffsetDateTime,
        pub sharing: Option<Sharing>,
        pub episode: Episode,
        pub show: Show,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, trakt_macros::Response)]
    #[trakt(expected = CREATED)]
    pub struct EpisodeResponse {
        pub id: u64,
        #[serde(with = "time::serde::iso8601")]
        pub watched_at: OffsetDateTime,
        pub sharing: Option<Sharing>,
        pub episode: Episode,
        pub show: Show,
    }
}

pub mod delete {
    //! Delete any active checkins
    //!
    //! <https://trakt.docs.apiary.io/#reference/checkin/checkin/delete-any-active-checkins>

    #[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/checkin",
    method = DELETE,
    auth = Required
    )]
    pub struct Request;

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = NO_CONTENT)]
    pub struct Response;
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use trakt_core::{Context, Request};

    use super::*;
    use crate::{
        smo::{Id, Sharing},
        test::assert_req,
    };

    const CTX: Context = Context {
        base_url: "https://api.trakt.tv",
        client_id: "client_id",
        oauth_token: Some("token"),
    };

    #[test]
    fn checkin_movie_request() {
        let expected = r#"{"movie":{"ids":{"trakt":1}}}"#;
        let request = checkin::Request::new_movie(Id::Trakt(1));
        assert_req!(CTX, request, "https://api.trakt.tv/checkin", expected);
    }

    #[test]
    fn checkin_movie_request_extra() {
        let expected = serde_json::to_string(&json!({
            "movie": {
                "ids": { "trakt": 1 },
            },
            "sharing": {
                "twitter": true,
                "mastodon": false,
                "tumblr": true,
            },
            "message": "Hello, world!",
        }))
        .unwrap();
        let mut request = checkin::Request::new_movie(Id::Trakt(1));
        request.sharing = Some(Sharing {
            twitter: true,
            mastodon: false,
            tumblr: true,
        });
        request.message = Some("Hello, world!".into());
        assert_req!(CTX, request, "https://api.trakt.tv/checkin", &expected);
    }

    #[test]
    fn checkin_missing_oauth() {
        let request = checkin::Request::new_movie(Id::Trakt(1));
        let err = request
            .try_into_http_request::<Vec<u8>>(Context {
                base_url: "https://api.trakt.tv",
                client_id: "client_id",
                oauth_token: None,
            })
            .unwrap_err();
        assert!(matches!(
            err,
            trakt_core::error::IntoHttpError::MissingToken
        ));
    }

    #[test]
    fn checkin_episode_request() {
        let expected = r#"{"episode":{"ids":{"imdb":"tt12345"}}}"#;
        let request = checkin::Request::new_episode(Id::Imdb("tt12345".into()));
        assert_req!(CTX, request, "https://api.trakt.tv/checkin", expected);
    }
}
