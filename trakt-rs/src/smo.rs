//! Standard Media Objects

mod de;
mod ser;

use serde::{Deserialize, Serialize};
use smallstr::SmallString;
use time::OffsetDateTime;
use trakt_core::EmojiString;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Id {
    Trakt(u64),
    Slug(SmallString<[u8; 16]>),
    Tvdb(u64),
    Imdb(SmallString<[u8; 16]>),
    Tmdb(u64),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Ids {
    pub trakt: Option<u64>,
    pub slug: Option<SmallString<[u8; 16]>>,
    pub tvdb: Option<u64>,
    pub imdb: Option<SmallString<[u8; 16]>>,
    pub tmdb: Option<u64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Movie {
    pub title: String,
    pub year: u16,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Show {
    pub title: String,
    pub year: u16,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Season {
    pub number: u16,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Episode {
    pub season: u16,
    pub number: u16,
    pub title: String,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub private: bool,
    pub name: String,
    pub vip: bool,
    pub vip_ep: bool,
    pub ids: Ids,
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Period {
    Daily,
    #[default]
    Weekly,
    Monthly,
    Yearly,
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

/// 2-letter country code
pub type Country = TwoLetter;

/// 2-letter language code
pub type Language = TwoLetter;

/// 2-letter Codes
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TwoLetter([u8; 2]);

impl TwoLetter {
    #[must_use]
    pub fn new(code: &str) -> Self {
        let mut bytes = [0; 2];
        bytes.copy_from_slice(code.as_bytes());
        Self(bytes)
    }

    /// Create a `TwoLetter` from bytes without checking if the bytes are valid UTF-8
    ///
    /// # Arguments
    ///
    /// * `bytes`: Bytes to convert to a `TwoLetter`
    ///
    /// # Safety
    ///
    /// The bytes must be valid UTF-8.
    #[must_use]
    pub const unsafe fn from_bytes_unchecked(bytes: [u8; 2]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    #[default]
    Newest,
    Oldest,
    Likes,
    Replies,
    Highest,
    Lowest,
    Plays,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Comment {
    pub id: u32,
    pub parent_id: Option<u32>,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    pub comment: EmojiString,
    pub spoiler: bool,
    pub review: bool,
    pub replies: u32,
    pub likes: u32,
    pub user_stats: UserStats,
    pub user: User,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct UserStats {
    pub rating: u8,
    pub play_count: u32,
    pub completed_count: u32,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct List {
    pub name: EmojiString,
    pub description: EmojiString,
    pub privacy: String,
    pub share_link: String,
    pub r#type: ListType,
    pub display_numbers: bool,
    pub allow_comments: bool,
    pub sort_by: String,
    pub sort_how: String,
    #[serde(with = "time::serde::iso8601")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub updated_at: OffsetDateTime,
    pub item_count: u64,
    pub comment_count: u64,
    pub likes: u64,
    pub ids: Ids,
    pub user: User,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListType {
    Personal,
    Official,
    Watchlist,
    Favorites,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Ratings {
    pub rating: f32,
    pub votes: u32,
    pub distribution: Distribution,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Distribution(pub [u32; 10]);
