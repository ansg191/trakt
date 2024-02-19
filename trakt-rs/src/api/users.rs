//! User endpoints
//!
//! <https://trakt.docs.apiary.io/#reference/users>

pub mod settings {
    //! Retrieve Settings
    //!
    //! <https://trakt.docs.apiary.io/#reference/users/settings/retrieve-settings>

    use std::collections::HashMap;

    use smol_str::SmolStr;
    use time::OffsetDateTime;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, trakt_macros::Request)]
    #[trakt(
    response = Response,
    endpoint = "/users/settings",
    auth = Required,
    )]
    pub struct Request;

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize, trakt_macros::Response)]
    pub struct Response {
        pub user: FullUser,
        pub account: Account,
        pub connections: Connections,
        pub sharing_text: SharingText,
        pub limits: Limits,
    }

    #[allow(clippy::struct_excessive_bools)]
    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    #[non_exhaustive]
    pub struct FullUser {
        pub username: SmolStr,
        pub private: bool,
        pub name: SmolStr,
        pub vip: bool,
        pub vip_ep: bool,
        pub ids: UserIds,
        #[serde(with = "time::serde::iso8601")]
        pub joined_at: OffsetDateTime,
        pub location: SmolStr,
        pub about: String,
        pub gender: SmolStr,
        pub age: u8,
        pub images: Images,
        pub vip_og: bool,
        pub vip_years: u8,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    #[non_exhaustive]
    pub struct UserIds {
        pub slug: SmolStr,
        pub uuid: String,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct Images {
        pub avatar: Avatar,
        #[serde(flatten)]
        pub extra: HashMap<String, serde_json::Value>,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    #[non_exhaustive]
    pub struct Avatar {
        pub full: String,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    #[non_exhaustive]
    pub struct Account {
        pub timezone: SmolStr,
        pub date_format: SmolStr,
        pub time_24hr: bool,
        pub cover_image: String,
    }

    #[derive(Debug, Clone, Eq, PartialEq, serde::Deserialize)]
    pub struct Connections(HashMap<String, bool>);

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    #[non_exhaustive]
    pub struct SharingText {
        pub watching: String,
        pub watched: String,
        pub rated: String,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    #[non_exhaustive]
    pub struct Limits {
        pub list: Limit,
        pub watchlist: Limit,
        pub favorites: Limit,
    }

    #[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
    pub struct Limit {
        pub count: Option<u32>,
        pub item_count: u32,
    }
}
