use std::{fmt::Formatter, ops::Deref};

use serde::{de::Error, Deserialize, Deserializer};

/// A string that deserializes strings containing emoji shortcodes into their
/// respective unicode characters.
///
/// Use `EmojiString::from` to create a new instance of `EmojiString` from a
/// `&str`, replacing any emoji shortcodes with their respective unicode
/// characters.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EmojiString(String);

impl Deref for EmojiString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for EmojiString {
    fn from(mut value: &str) -> Self {
        let mut o = String::new();

        // Shamelessly stolen from:
        // https://github.com/rossmacarthur/emojis/blob/b088b129c59124df8c3d7c3a5aff116114c78acf/examples/replace.rs#L23
        // The meaning of the index values is as follows.
        //
        //  : r o c k e t :
        // ^ ^           ^ ^
        // i m           n j
        //
        // i..j gives ":rocket:"
        // m..n gives "rocket"
        while let Some((i, m, n, j)) = value
            .find(':')
            .map(|i| (i, i + 1))
            .and_then(|(i, m)| value[m..].find(':').map(|x| (i, m, m + x, m + x + 1)))
        {
            if let Some(emoji) = emojis::get_by_shortcode(&value[m..n]) {
                // Output everything preceding, except the first colon
                o.push_str(&value[..i]);
                // Output the emoji.
                o.push_str(emoji.as_str());
                // Update the string to past the last colon.
                value = &value[j..];
            } else {
                // Output everything preceding but not including the colon
                o.push_str(&value[..n]);
                // Update the string to start with the last colon
                value = &value[n..];
            }
        }

        o.push_str(value);
        Self(o)
    }
}

impl From<EmojiString> for String {
    fn from(value: EmojiString) -> Self {
        value.0
    }
}

impl<'de> Deserialize<'de> for EmojiString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = EmojiString;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a string containing emoji shortcodes")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(v.into())
            }

            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: Error,
            {
                let s = std::str::from_utf8(v)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Bytes(v), &self))?;
                Ok(s.into())
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_from_str() {
        let tests = [
            ("launch nothing", "launch nothing"),
            ("launch :rocket: something", "launch 🚀 something"),
            ("? :unknown: emoji", "? :unknown: emoji"),
            ("::very:naughty::", "::very:naughty::"),
            (":maybe:rocket:", ":maybe🚀"),
            (":rocket::rocket:", "🚀🚀"),
        ];

        for (i, o) in tests {
            let i: EmojiString = i.into();
            assert_eq!(&*i, o);
        }
    }

    #[test]
    pub fn test_deserialize() {
        let tests = [
            ("launch nothing", "launch nothing"),
            ("launch :rocket: something", "launch 🚀 something"),
            ("? :unknown: emoji", "? :unknown: emoji"),
            ("::very:naughty::", "::very:naughty::"),
            (":maybe:rocket:", ":maybe🚀"),
            (":rocket::rocket:", "🚀🚀"),
        ];

        for (i, o) in tests {
            let i: EmojiString = serde_json::from_str(&format!("\"{i}\"")).unwrap();
            assert_eq!(&*i, o);
        }
    }
}
