//! Comments
//!
//! <https://trakt.docs.apiary.io/#reference/comments>

pub mod post {
    //! Post a comments
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/comments/post-a-comment>

    use bytes::BufMut;
    use serde_json::{json, Value};
    use trakt_core::{error::IntoHttpError, Context, Metadata};
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

            trakt_core::construct_req(&ctx, &Self::METADATA, &(), &(), writer.into_inner())
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = CREATED)]
    pub struct Response(pub Comment);
}

pub mod get {
    //! Get a comment or reply
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/comment/get-a-comment-or-reply>

    use crate::smo::Comment;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/{id}",
    )]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Comment);
}

pub mod update {
    //! Update a comment or repl
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/comment/update-a-comment-or-reply>

    use bytes::BufMut;
    use serde::Serialize;
    use serde_json::json;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    use crate::smo::Comment;

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Request {
        pub id: u64,
        pub comment: String,
        pub spoiler: bool,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize)]
    struct RequestParams {
        id: u64,
    }

    impl trakt_core::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/comments/{id}",
            method: http::Method::PUT,
            auth: trakt_core::AuthRequirement::Required,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let body = T::default();
            let mut writer = body.writer();

            let json = json!({
                "comment": self.comment,
                "spoiler": self.spoiler,
            });
            serde_json::to_writer(&mut writer, &json)?;

            let params = RequestParams { id: self.id };
            trakt_core::construct_req(&ctx, &Self::METADATA, &params, &(), writer.into_inner())
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Comment);
}

pub mod delete {
    //! Delete a comment or reply
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/comment/delete-a-comment-or-reply>

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/{id}",
    method = DELETE,
    auth = Required,
    )]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = NO_CONTENT)]
    pub struct Response;
}

pub mod get_replies {
    //! Get comment replies
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/comment/get-replies-for-a-comment>

    use crate::smo::Comment;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/{id}/replies",
    auth = Optional,
    )]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Vec<Comment>);
}

pub mod post_reply {
    //! Post a reply for a comment
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/replies/post-a-reply-for-a-comment>

    use bytes::BufMut;
    use serde::Serialize;
    use serde_json::json;
    use trakt_core::{error::IntoHttpError, Context, Metadata};

    use crate::smo::Comment;

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Request {
        pub id: u64,
        pub comment: String,
        pub spoiler: bool,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize)]
    struct RequestParams {
        id: u64,
    }

    impl trakt_core::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/comments/{id}/replies",
            method: http::Method::POST,
            auth: trakt_core::AuthRequirement::Required,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let body = T::default();
            let mut writer = body.writer();

            let json = json!({
                "comment": self.comment,
                "spoiler": self.spoiler,
            });
            serde_json::to_writer(&mut writer, &json)?;

            let params = RequestParams { id: self.id };
            trakt_core::construct_req(&ctx, &Self::METADATA, &params, &(), writer.into_inner())
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = CREATED)]
    pub struct Response(pub Comment);
}

pub mod item {
    //! Get attached media item for a comment
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/item/get-the-attached-media-item>

    use crate::smo::Item;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/{id}/item",
    )]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response(pub Item);
}

pub mod likes {
    //! Get users who liked a comment
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/likes/get-all-users-who-liked-a-comment>

    use time::OffsetDateTime;
    use trakt_core::PaginationResponse;

    use crate::smo::User;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/{id}/likes",
    )]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub users: PaginationResponse<ResponseItem>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct ResponseItem {
        #[serde(with = "time::serde::iso8601")]
        pub liked_at: OffsetDateTime,
        pub user: User,
    }
}

pub mod like {
    //! Like a comment
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/like/like-a-comment>

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/{id}/like",
    method = POST,
    auth = Required,
    )]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = NO_CONTENT)]
    pub struct Response;
}

pub mod remove_like {
    //! Remove like from a comment
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/like/remove-like-on-a-comment>

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/{id}/like",
    method = DELETE,
    auth = Required,
    )]
    pub struct Request {
        pub id: u64,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    #[trakt(expected = NO_CONTENT)]
    pub struct Response;
}

pub mod trending {
    //! Get trending comments
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/like/get-trending-comments>

    use trakt_core::PaginationResponse;

    use crate::smo::{CommentItemType, CommentType, CommentWithItem};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/trending/{comment_type}/{tp}",
    )]
    pub struct Request {
        pub comment_type: CommentType,
        pub tp: CommentItemType,
        pub include_replies: bool,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub comments: PaginationResponse<CommentWithItem>,
    }
}

pub mod recent {
    //! Get recently created comments
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/trending/get-recently-created-comments>

    use trakt_core::PaginationResponse;

    use crate::smo::{CommentItemType, CommentType, CommentWithItem};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/recent/{comment_type}/{tp}",
    )]
    pub struct Request {
        pub comment_type: CommentType,
        pub tp: CommentItemType,
        pub include_replies: bool,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub comments: PaginationResponse<CommentWithItem>,
    }
}

pub mod recent_updated {
    //! Get recently updated comments
    //!
    //! <https://trakt.docs.apiary.io/#reference/comments/updates/get-recently-updated-comments>

    use trakt_core::PaginationResponse;

    use crate::smo::{CommentItemType, CommentType, CommentWithItem};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/comments/updates/{comment_type}/{tp}",
    )]
    pub struct Request {
        pub comment_type: CommentType,
        pub tp: CommentItemType,
        pub include_replies: bool,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub comments: PaginationResponse<CommentWithItem>,
    }
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

    #[test]
    fn update_comment_request() {
        const COMMENT: &str = "The quick brown fox jumps over the lazy dog.";

        let expected = json!({
            "comment": COMMENT,
            "spoiler": false,
        });
        let request = update::Request {
            id: 42,
            comment: COMMENT.to_owned(),
            spoiler: false,
        };
        assert_request(CTX, request, "https://api.trakt.tv/comments/42", &expected);
    }
}
