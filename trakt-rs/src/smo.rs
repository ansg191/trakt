//! Standard Media Objects

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Ids {
    pub trakt: Option<u64>,
    pub slug: Option<String>,
    pub tvdb: Option<u64>,
    pub imdb: Option<String>,
    pub tmdb: Option<u64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: u16,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: u16,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Season {
    pub number: u16,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Episode {
    pub season: u16,
    pub number: u16,
    pub title: String,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub private: bool,
    pub name: String,
    pub vip: bool,
    pub vip_ep: bool,
    pub ids: Ids,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize)]
pub enum Period {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "yearly")]
    Yearly,
    #[serde(rename = "all")]
    All,
}

impl Period {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
            Self::Yearly => "yearly",
            Self::All => "all",
        }
    }
}

