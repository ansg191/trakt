//! Comments
//!
//! <https://trakt.docs.apiary.io/#reference/comments>

pub mod post {
    //! Post a comments
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/comments/post-a-comment>

    use bytes::BufMut;
    use serde_json::{json, Value};
    use trakt_core::{construct_url, error::IntoHttpError, Context, Metadata};
    use unicode_segmentation::UnicodeSegmentation;

    use crate::smo::{Comment, Id, Ids, Sharing};

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Request {
        pub tp: Type,
        pub id: Id,
        pub comment: String,
        pub spoiler: bool,
        pub sharing: Option<Sharing>,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Type {
        Movie,
        Show,
        Season,
        Episode,
        List,
    }

    impl trakt_core::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/comments",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::Required,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            // Check that comments have at least 5 words
            if self.comment.unicode_words().count() < 5 {
                return Err(IntoHttpError::Validation(
                    "Comments must be at least 5 words long".to_owned(),
                ));
            }

            let url = construct_url(ctx.base_url, Self::METADATA.endpoint, &(), &())?;

            let body = T::default();
            let mut writer = body.writer();

            let json = Value::Object({
                let mut map = serde_json::Map::new();
                map.insert("comment".to_owned(), Value::String(self.comment));
                map.insert("spoiler".to_owned(), Value::Bool(self.spoiler));
                if let Some(sharing) = self.sharing {
                    map.insert("sharing".to_owned(), json!(sharing));
                }

                let id = json!({ "ids": Ids::from(self.id) });
                match self.tp {
                    Type::Movie => map.insert("movie".to_owned(), id),
                    Type::Show => map.insert("show".to_owned(), id),
                    Type::Season => map.insert("season".to_owned(), id),
                    Type::Episode => map.insert("episode".to_owned(), id),
                    Type::List => map.insert("list".to_owned(), id),
                };
                map
            });
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

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = CREATED)]
    pub struct Response(pub Comment);
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use trakt_core::{Context, Request};

    use super::*;
    use crate::{
        smo::{
            Id::{Imdb, Slug, Trakt},
            Sharing,
        },
        test::assert_request,
    };

    const CTX: Context = Context {
        base_url: "https://api.trakt.tv",
        client_id: "client_id",
        oauth_token: Some("token"),
    };

    #[test]
    fn post_comment_request() {
        const COMMENT: &str = "The quick brown fox jumps over the lazy dog.";

        let expected = json!({
            "movie": { "ids": { "trakt": 1 } },
            "comment": COMMENT,
            "spoiler": false,
        });
        let request = post::Request {
            tp: post::Type::Movie,
            id: Trakt(1),
            comment: COMMENT.to_owned(),
            spoiler: false,
            sharing: None,
        };
        assert_request(CTX, request, "https://api.trakt.tv/comments", &expected);
    }

    #[test]
    fn post_comment_request_bad_comment() {
        const COMMENT: &str = "The quick brown fox";

        let request = post::Request {
            tp: post::Type::Show,
            id: Imdb("tt1234567".into()),
            comment: COMMENT.to_owned(),
            spoiler: false,
            sharing: None,
        };
        let result = request.try_into_http_request::<Vec<u8>>(CTX);
        assert!(result.is_err());
        assert!(matches!(
            result,
            Err(trakt_core::error::IntoHttpError::Validation(_))
        ));
    }

    #[test]
    fn post_comment_request_sharing() {
        const COMMENT: &str = "The quick brown fox jumps over the lazy dog.";

        let expected = json!({
            "episode": { "ids": { "slug": "slug" } },
            "comment": COMMENT,
            "spoiler": false,
            "sharing": {
                "twitter": false,
                "mastodon": true,
                "tumblr": false,
            },
        });
        let request = post::Request {
            tp: post::Type::Episode,
            id: Slug("slug".into()),
            comment: COMMENT.to_owned(),
            spoiler: false,
            sharing: Some(Sharing {
                twitter: false,
                mastodon: true,
                tumblr: false,
            }),
        };
        assert_request(CTX, request, "https://api.trakt.tv/comments", &expected);
    }
}
