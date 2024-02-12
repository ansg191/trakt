//! Standard Media Objects

use std::fmt::Formatter;

use serde::{de::Unexpected, Deserialize, Deserializer, Serialize, Serializer};

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
#[serde(rename_all = "lowercase")]
pub enum Period {
    Daily,
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

    #[must_use]
    pub const unsafe fn from_bytes_unchecked(bytes: [u8; 2]) -> Self {
        Self(bytes)
    }

    #[must_use]
    pub const fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl Serialize for Country {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for TwoLetter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'a> serde::de::Visitor<'a> for Visitor {
            type Value = TwoLetter;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a 2 letter country code")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if value.len() != 2 {
                    return Err(E::invalid_length(value.len(), &"2"));
                }
                Ok(TwoLetter::new(value))
            }

            fn visit_borrowed_bytes<E>(self, v: &'a [u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() != 2 {
                    return Err(E::invalid_length(v.len(), &"2"));
                }
                let s = std::str::from_utf8(v)
                    .map_err(|_| E::invalid_value(Unexpected::Bytes(v), &self))?;
                Ok(TwoLetter::new(s))
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}
