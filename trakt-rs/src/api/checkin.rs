//! Checkin
//!
//! <https://trakt.docs.apiary.io/#reference/checkin>

pub mod check_in {
    //! Check into an item
    //!
    //! <https://trakt.docs.apiary.io/#reference/checkin/checkin/check-into-an-item>

    use bytes::BufMut;
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use time::OffsetDateTime;
    use trakt_core::{construct_url, error::IntoHttpError, AuthRequirement, Context, Metadata};

    use crate::smo::{Episode, Id, Movie, Show};

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Request<I: CheckinItem> {
        pub id: Id,
        pub sharing: Option<Sharing>,
        pub message: Option<String>,
        _phantom: std::marker::PhantomData<I>,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
    pub struct Sharing {
        twitter: bool,
        mastodon: bool,
        tumblr: bool,
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
            let url = construct_url(ctx.base_url, Self::METADATA.endpoint, &(), &())?;

            let body = T::default();
            let mut writer = body.writer();

            let id = match &self.id {
                Id::Slug(slug) => json!({ "ids": { "slug": slug } }),
                Id::Trakt(trakt) => json!({ "ids": { "trakt": trakt } }),
                Id::Imdb(imdb) => json!({ "ids": { "imdb": imdb } }),
                Id::Tmdb(tmdb) => json!({ "ids": { "tmdb": tmdb } }),
                Id::Tvdb(tvdb) => json!({ "ids": { "tvdb": tvdb } }),
            };

            let mut json = json!({
                "sharing": self.sharing,
                "message": self.message,
            });
            // Won't panic because json is an object
            json.as_object_mut().unwrap().insert(I::KEY.to_owned(), id);

            serde_json::to_writer(&mut writer, &json)?;

            let request = http::Request::builder()
                .method(Self::METADATA.method)
                .uri(url)
                .header("Content-Type", "application/json")
                .header("trakt-api-version", "2")
                .header("trakt-api-key", ctx.client_id);
            let request = match ctx.oauth_token {
                Some(token) => request.header("Authorization", format!("Bearer {token}")),
                None => return Err(IntoHttpError::MissingToken),
            };

            Ok(request.body(writer.into_inner())?)
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
