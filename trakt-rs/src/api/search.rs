//! Search endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/search>

use serde::Serializer;

use crate::smo::Item;

bitflags::bitflags! {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct SearchType: u8 {
        const MOVIE = 0b0000_0001;
        const SHOW = 0b0000_0010;
        const EPISODE = 0b0000_0100;
        const PERSON = 0b0000_1000;
        const LIST = 0b0001_0000;
    }
}

impl serde::Serialize for SearchType {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        const FLAGS: [&str; 5] = ["movie", "show", "episode", "person", "list"];

        if self.is_empty() {
            serializer.serialize_none()
        } else if self.bits().count_ones() == 1 {
            // Serialize as a single value

            // Get name of the flag
            let idx = self.bits().trailing_zeros() as usize;
            serializer.serialize_str(FLAGS[idx])
        } else {
            // Serialize as a comma-separated list
            // We can't serialize as a sequence b/c serde_urlencoded doesn't support it

            // Get names of the flags
            let iter = self.iter().map(|flag| {
                let idx = flag.bits().trailing_zeros() as usize;
                FLAGS[idx]
            });

            // Join the names
            let joined = iter.collect::<Vec<_>>().join(",");

            serializer.serialize_str(&joined)
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct SearchResult {
    #[serde(flatten)]
    pub item: Item,
    pub score: Option<f64>,
}

pub mod text_query {
    //! Text query search
    //!
    //! <https://trakt.docs.apiary.io/#reference/search/text-query/get-text-query-results>

    use trakt_core::{Pagination, PaginationResponse};

    use super::{SearchResult, SearchType};

    #[derive(Debug, Clone, PartialEq, Eq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/search/{tp}"
    )]
    pub struct Request {
        pub tp: SearchType,
        pub query: String,
        #[serde(flatten)]
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, PartialEq, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<SearchResult>,
    }
}

pub mod id_lookup {
    //! Lookup items by their IDs
    //!
    //! <https://trakt.docs.apiary.io/#reference/search/text-query/get-id-lookup-results>

    use bytes::BufMut;
    use serde::Serialize;
    use trakt_core::{error::IntoHttpError, Context, Metadata, Pagination, PaginationResponse};

    use super::{SearchResult, SearchType};
    use crate::smo::Id;

    #[derive(Debug, Clone, Eq, PartialEq, Hash)]
    pub struct Request {
        pub id: Id,
        pub tp: SearchType,
        pub pagination: Pagination,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
    struct RequestPathParams {
        id_type: &'static str,
        id: Id,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize)]
    struct RequestQueryParams {
        #[serde(rename = "type")]
        tp: SearchType,
        #[serde(flatten)]
        pagination: Pagination,
    }

    impl TryFrom<Request> for (RequestPathParams, RequestQueryParams) {
        type Error = IntoHttpError;

        fn try_from(value: Request) -> Result<Self, Self::Error> {
            Ok((
                RequestPathParams {
                    id_type: match &value.id {
                        Id::Trakt(_) => "trakt",
                        Id::Slug(_) => {
                            return Err(IntoHttpError::Validation(String::from(
                                "Slug IDs are not supported",
                            )));
                        }
                        Id::Tvdb(_) => "tvdb",
                        Id::Imdb(_) => "imdb",
                        Id::Tmdb(_) => "tmdb",
                    },
                    id: value.id,
                },
                RequestQueryParams {
                    tp: value.tp,
                    pagination: value.pagination,
                },
            ))
        }
    }

    impl trakt_core::Request for Request {
        type Response = Response;
        const METADATA: Metadata = Metadata {
            endpoint: "/search/{id_type}/{id}",
            method: http::Method::GET,
            auth: trakt_core::AuthRequirement::None,
        };

        fn try_into_http_request<T: Default + BufMut>(
            self,
            ctx: Context,
        ) -> Result<http::Request<T>, IntoHttpError> {
            let (path, query) = self.try_into()?;
            trakt_core::construct_req(&ctx, &Self::METADATA, &path, &query, T::default())
        }
    }

    #[derive(Debug, Clone, PartialEq, trakt_macros::Response)]
    pub struct Response {
        #[trakt(pagination)]
        pub items: PaginationResponse<SearchResult>,
    }
}

#[cfg(test)]
mod tests {
    use trakt_core::{construct_url, error::IntoHttpError, Context, Pagination, Request};

    use super::*;
    use crate::{smo::Id, test::assert_request};

    const CTX: Context = Context {
        base_url: "https://api.trakt.tv",
        client_id: "client_id",
        oauth_token: None,
    };

    #[test]
    fn test_type_ser() {
        let tp = SearchType::MOVIE;
        assert_eq!(serde_json::to_string(&tp).unwrap(), r#""movie""#);

        let tp = SearchType::MOVIE | SearchType::SHOW;
        assert_eq!(serde_json::to_string(&tp).unwrap(), r#""movie,show""#);

        let tp = SearchType::empty();
        assert_eq!(serde_json::to_string(&tp).unwrap(), "null");
    }

    #[test]
    fn test_type_ser_url() {
        #[derive(Debug, serde::Serialize)]
        struct Test {
            tp: SearchType,
        }

        let test = Test {
            tp: SearchType::MOVIE,
        };
        let url = construct_url("", "/search/{tp}", &test, &()).unwrap();
        assert_eq!(url, "/search/movie");

        let test = Test {
            tp: SearchType::MOVIE | SearchType::SHOW,
        };
        let url = construct_url("", "/search/{tp}", &test, &()).unwrap();
        assert_eq!(url, "/search/movie,show");

        let test = Test {
            tp: SearchType::empty(),
        };
        let url = construct_url("", "/search/{tp}", &test, &()).unwrap();
        assert_eq!(url, "/search/");
    }

    #[test]
    fn test_id_lookup_request() {
        let req = id_lookup::Request {
            id: Id::Trakt(1),
            tp: SearchType::MOVIE,
            pagination: Pagination::default(),
        };
        assert_request(
            CTX,
            req,
            "https://api.trakt.tv/search/trakt/1?type=movie&page=1&limit=10",
            "",
        );

        let req = id_lookup::Request {
            id: Id::Tvdb(1),
            tp: SearchType::EPISODE | SearchType::SHOW,
            pagination: Pagination::default(),
        };
        assert_request(
            CTX,
            req,
            "https://api.trakt.tv/search/tvdb/1?type=show%2Cepisode&page=1&limit=10",
            "",
        );

        let req = id_lookup::Request {
            id: Id::Imdb("tt12345".into()),
            tp: SearchType::empty(),
            pagination: Pagination::default(),
        };
        assert_request(
            CTX,
            req,
            "https://api.trakt.tv/search/imdb/tt12345?page=1&limit=10",
            "",
        );

        let req = id_lookup::Request {
            id: Id::Slug("slug".into()),
            tp: SearchType::PERSON,
            pagination: Pagination::default(),
        };
        assert!(matches!(
            req.try_into_http_request::<Vec<u8>>(CTX),
            Err(IntoHttpError::Validation(_))
        ));
    }
}
