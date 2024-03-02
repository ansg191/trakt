//! Standard Media Objects

mod de;
mod ser;

use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use trakt_core::EmojiString;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
#[serde(untagged)]
pub enum Id {
    Trakt(u64),
    Slug(CompactString),
    Tvdb(u64),
    Imdb(CompactString),
    Tmdb(u64),
}

impl From<Id> for Ids {
    fn from(value: Id) -> Self {
        let mut ret = Self::default();
        match value {
            Id::Trakt(trakt) => ret.trakt = Some(trakt),
            Id::Slug(slug) => ret.slug = Some(slug),
            Id::Tvdb(tvdb) => ret.tvdb = Some(tvdb),
            Id::Imdb(imdb) => ret.imdb = Some(imdb),
            Id::Tmdb(tmdb) => ret.tmdb = Some(tmdb),
        }
        ret
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Ids {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trakt: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tvdb: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imdb: Option<CompactString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmdb: Option<u64>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Movie {
    pub title: CompactString,
    pub year: u16,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Show {
    pub title: CompactString,
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
    pub title: CompactString,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Person {
    pub name: CompactString,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct User {
    pub username: CompactString,
    pub private: bool,
    pub name: CompactString,
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
        unsafe { Self::from_bytes_unchecked(bytes) }
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
    pub sharing: Option<Sharing>,
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
    pub privacy: ListPrivacy,
    pub share_link: String,
    pub r#type: ListType,
    pub display_numbers: bool,
    pub allow_comments: bool,
    pub sort_by: ListSortBy,
    pub sort_how: ListSortHow,
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ListSortBy {
    Rank,
    Added,
    Title,
    Released,
    Runtime,
    Popularity,
    Percentage,
    Votes,
    MyRating,
    Random,
    Watched,
    Collected,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListSortHow {
    Asc,
    Desc,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListPrivacy {
    #[default]
    Private,
    Link,
    Friends,
    Public,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Ratings {
    pub rating: f32,
    pub votes: u32,
    pub distribution: Distribution,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Distribution(pub [u32; 10]);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct Studio {
    pub name: CompactString,
    pub country: Country,
    pub ids: Ids,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct EpisodeAirEvent {
    #[serde(with = "time::serde::iso8601")]
    pub first_aired: OffsetDateTime,
    pub episode: Episode,
    pub show: Show,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
pub struct MovieReleaseEvent {
    #[serde(with = "crate::iso8601_date")]
    pub release_date: Date,
    pub movie: Movie,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize, Deserialize)]
pub struct Sharing {
    pub twitter: bool,
    pub mastodon: bool,
    pub tumblr: bool,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentType {
    #[default]
    All,
    Reviews,
    Shouts,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentItemType {
    #[default]
    All,
    Movies,
    Shows,
    Seasons,
    Episodes,
    Lists,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum CommentWithItem {
    Movie {
        movie: Box<Movie>,
        comment: Comment,
    },
    Show {
        show: Box<Show>,
        comment: Comment,
    },
    Season {
        season: Box<Season>,
        comment: Comment,
    },
    Episode {
        episode: Box<Episode>,
        comment: Comment,
    },
    List {
        list: Box<List>,
        comment: Comment,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "type")]
pub enum Item {
    Movie { movie: Box<Movie> },
    Show { show: Box<Show> },
    Season { season: Box<Season> },
    Episode { episode: Box<Episode> },
    List { list: Box<List> },
}
