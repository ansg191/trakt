//! Genre related endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/genres>

pub mod list {
    //! Get genres
    //!
    //! <https://trakt.docs.apiary.io/#reference/genres/list/get-genres>

    use compact_str::CompactString;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/genres/{tp}",
    )]
    pub struct Request {
        pub tp: Type,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Type {
        Movies,
        Shows,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Hash, trakt_macros::Response)]
    pub struct Response(Vec<ResponseItem>);

    #[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
    pub struct ResponseItem {
        pub name: CompactString,
        pub slug: CompactString,
    }
}
