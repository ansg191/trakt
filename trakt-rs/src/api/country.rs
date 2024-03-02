//! Country-related endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/countries/list>

pub mod list {
    //! List all countries
    //!
    //! <https://trakt.docs.apiary.io/#reference/countries/list/get-countries>

    use compact_str::CompactString;
    use serde::{Deserialize, Serialize};

    use crate::smo::Country;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/countries/{tp}",
    )]
    pub struct Request {
        tp: Type,
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
        pub code: Country,
    }
}
