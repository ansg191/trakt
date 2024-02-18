//! Certifications
//!
//! <https://trakt.docs.apiary.io/#reference/certifications/list>

pub mod list {
    //! List Certifications
    //!
    //! <https://trakt.docs.apiary.io/#reference/certifications/list/get-certifications>

    use std::collections::HashMap;

    use serde::Serialize;
    use smol_str::SmolStr;

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
        pub name: SmolStr,
        pub slug: SmolStr,
        pub description: String,
    }
}
