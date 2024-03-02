//! Certifications
//!
//! <https://trakt.docs.apiary.io/#reference/certifications/list>

pub mod list {
    //! List Certifications
    //!
    //! <https://trakt.docs.apiary.io/#reference/certifications/list/get-certifications>

    use std::collections::HashMap;

    use compact_str::CompactString;
    use serde::Serialize;

    use crate::smo::Country;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/certifications/{tp}",
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

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, trakt_macros::Response)]
    pub struct Response(pub HashMap<Country, Certification>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct Certification {
        pub name: CompactString,
        pub slug: CompactString,
        pub description: CompactString,
    }
}
