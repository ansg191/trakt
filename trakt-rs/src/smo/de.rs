use std::fmt::Formatter;

use serde::{
    de::{Error, MapAccess, SeqAccess, Unexpected},
    Deserialize, Deserializer,
};

use super::{Distribution, TwoLetter};

impl<'de> Deserialize<'de> for TwoLetter {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor;

        impl<'a> serde::de::Visitor<'a> for Visitor {
            type Value = TwoLetter;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a 2 letter country code")
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                if value.len() != 2 {
                    return Err(E::invalid_length(value.len(), &"2"));
                }
                Ok(TwoLetter::new(value))
            }

            fn visit_borrowed_bytes<E: Error>(self, v: &'a [u8]) -> Result<Self::Value, E> {
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

impl<'de> Deserialize<'de> for Distribution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
        struct Field(u8);

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> serde::de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        formatter.write_str("a number between 1 and 10")
                    }

                    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
                        if (1..=10).contains(&v) {
                            // Won't panic because we checked the range
                            Ok(Field(u8::try_from(v - 1).unwrap()))
                        } else {
                            Err(E::invalid_value(Unexpected::Signed(v), &"1-10"))
                        }
                    }

                    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
                        if (1..=10).contains(&v) {
                            // Won't panic because we checked the range
                            Ok(Field(u8::try_from(v - 1).unwrap()))
                        } else {
                            Err(E::invalid_value(Unexpected::Unsigned(v), &"1-10"))
                        }
                    }

                    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
                        v.parse().map_or_else(
                            |_| Err(E::unknown_field(v, FIELDS)),
                            |n| self.visit_u64(n),
                        )
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DistributionVisitor;

        impl<'de> serde::de::Visitor<'de> for DistributionVisitor {
            type Value = Distribution;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("a distribution of ratings")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
                let mut distribution = [0; 10];
                let mut i = 0;
                while let Some(value) = seq.next_element()? {
                    distribution[i] = value;
                    i += 1;
                }
                Ok(Distribution(distribution))
            }

            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut distribution = [0; 10];
                while let Some(key) = map.next_key::<Field>()? {
                    distribution[usize::from(key.0)] = map.next_value()?;
                }
                Ok(Distribution(distribution))
            }
        }

        deserializer.deserialize_struct("Distribution", FIELDS, DistributionVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn two_letter() {
        let json = r#""de""#;
        let two: TwoLetter = serde_json::from_str(json).unwrap();
        assert_eq!(two, TwoLetter::new("de"));

        let json = r#""d""#;
        let two: Result<TwoLetter, _> = serde_json::from_str(json);
        assert!(two.is_err());

        let json = r#""deu""#;
        let two: Result<TwoLetter, _> = serde_json::from_str(json);
        assert!(two.is_err());

        let json = br#""de""#;
        let two: TwoLetter = serde_json::from_slice(json).unwrap();
        assert_eq!(two, TwoLetter::new("de"));

        let json = b"\xc3\x28";
        let two: Result<TwoLetter, _> = serde_json::from_slice(json);
        assert!(two.is_err());
    }

    #[test]
    fn distribution() {
        let json = json!({
            "1": 1,
            "2": 2,
            "3": 3,
            "4": 4,
            "5": 5,
            "6": 6,
            "7": 7,
            "8": 8,
            "9": 9,
            "10": 10
        });
        let dist: Distribution = serde_json::from_value(json).unwrap();
        assert_eq!(dist, Distribution([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

        let json = json!([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        let dist: Distribution = serde_json::from_value(json).unwrap();
        assert_eq!(dist, Distribution([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]));

        let json = json!({
            "1": 1,
            "2": 2,
            "3": 3,
            "4": 4,
            "5": 5,
            "6": 6,
            "7": 7,
            "8": 8,
            "9": 9
        });
        let dist: Distribution = serde_json::from_value(json).unwrap();
        assert_eq!(dist, Distribution([1, 2, 3, 4, 5, 6, 7, 8, 9, 0]));

        let json = json!({
            "1": 1,
            "a": 2,
        });
        let dist: Result<Distribution, _> = serde_json::from_value(json);
        assert!(dist.is_err());

        let json = json!({
            "1": 1,
            "2": 2,
            "3": 3,
            "4": 4,
            "5": 5,
            "6": 6,
            "7": 7,
            "8": 8,
            "9": 9,
            "10": 10,
            "11": 11,
        });
        let dist: Result<Distribution, _> = serde_json::from_value(json);
        assert!(dist.is_err());
    }
}
