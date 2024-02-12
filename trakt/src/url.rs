use std::fmt::Display;

use percent_encoding::{AsciiSet, CONTROLS};
use serde::{ser, Serialize};

use crate::IntoHttpError;

pub fn construct_url(
    base_url: &str,
    endpoint: &str,
    params: &impl Serialize,
    query: &impl Serialize,
) -> Result<String, IntoHttpError> {
    // Serialize the url parameters
    let url = to_string(base_url, endpoint, params)?;

    // Serialize the query parameters
    let query = serde_urlencoded::to_string(query)?;

    // If there are query parameters, append them to the URL
    let url = if query.is_empty() {
        url
    } else {
        format!("{url}?{query}")
    };

    Ok(url)
}

struct UrlSerializer<'a> {
    /// The URL being built
    url: String,
    /// The parts of the URL endpoint
    parts: Vec<Part<'a>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Part<'a> {
    /// A raw string that should be appended to the URL
    Raw(&'a str),
    /// A parameter that should be URL encoded and appended to the URL.
    Param(Param<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Param<'a> {
    /// The key of the parameter
    Key(&'a str),
    /// The serialized value of the parameter
    Value(String),
}

fn to_string<T: Serialize>(base_url: &str, endpoint: &str, value: &T) -> Result<String, Error> {
    let mut serializer = UrlSerializer {
        url: base_url.to_owned(),
        parts: parse_endpoint(endpoint)?,
    };
    value.serialize(&mut serializer)?;
    serializer.end()
}

/// Parses the endpoint into parts
///
/// Example endpoint: `/shows/{id}/seasons/{season}/episodes/{episode}`
///
/// Example parts:
/// - `Raw("/shows/")`
/// - `Param("id")`
/// - `Raw("/seasons/")`
/// - `Param("season")`
/// - `Raw("/episodes/")`
/// - `Param("episode")`
fn parse_endpoint(s: &str) -> Result<Vec<Part>, Error> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut in_param = false;
    for (i, c) in s.char_indices() {
        // Find the start of a parameter
        if c == '{' {
            // If we're already in a parameter, this is an error
            if in_param {
                return Err(Error::InvalidEndpoint);
            }

            // Mark that we're in a parameter
            in_param = true;

            // If there's a string before this parameter, add it to the parts
            if start != i {
                parts.push(Part::Raw(&s[start..i]));
            }

            // Move the start to the beginning of the parameter
            start = i + 1;
        } else if c == '}' {
            // If we're not in a parameter, this is an error
            if !in_param {
                return Err(Error::InvalidEndpoint);
            }

            // Mark that we're no longer in a parameter
            in_param = false;

            // Add the parameter to the parts
            if start != i {
                parts.push(Part::Param(Param::Key(&s[start..i])));
            }

            // Move the start to the end of the parameter
            start = i + 1;
        }
    }

    // If we're still in a parameter at end of endpoint, this is an error
    if in_param {
        return Err(Error::InvalidEndpoint);
    }

    // Add the last part of the string to the parts
    if start != s.len() {
        parts.push(Part::Raw(&s[start..]));
    }

    Ok(parts)
}

impl<'a> UrlSerializer<'a> {
    pub fn end(self) -> Result<String, Error> {
        let mut url = self.url;
        for part in self.parts {
            match part {
                Part::Raw(s) => url.push_str(s),
                Part::Param(p) => match p {
                    Param::Key(k) => return Err(Error::UnfilledField(k.to_owned())),
                    Param::Value(v) => url.push_str(&v),
                },
            }
        }
        Ok(url)
    }
}

impl<'a, 'b> ser::Serializer for &'a mut UrlSerializer<'b> {
    type Ok = ();

    type Error = Error;
    type SerializeSeq = ErrorSerializer;
    type SerializeTuple = ErrorSerializer;
    type SerializeTupleStruct = ErrorSerializer;
    type SerializeTupleVariant = ErrorSerializer;
    type SerializeMap = ErrorSerializer;
    type SerializeStruct = Self;
    type SerializeStructVariant = ErrorSerializer;

    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_i32(self, _v: i32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_i64(self, _v: i64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_u32(self, _v: u32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_u64(self, _v: u64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_char(self, _v: char) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_str(self, _v: &str) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        Err(Error::TopLevel)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::ValueNotSupported)
    }
}

impl<'a, 'b> ser::SerializeStruct for &'a mut UrlSerializer<'b> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error> {
        // Search for the key in the parts
        let mut part = None;
        for p in &mut self.parts {
            match p {
                Part::Param(p) => match p {
                    Param::Key(k) if *k == key => {
                        part = Some(p);
                        break;
                    }
                    _ => {}
                },
                Part::Raw(_) => {}
            }
        }

        // If the key was not found, this is an error
        let part = part.ok_or(Error::KeyNotFound(key))?;

        // Serialize the value into the part
        let mut serializer = UrlValueSerializer::default();
        value.serialize(&mut serializer)?;
        let value = serializer.value;

        *part = Param::Value(value);

        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
struct UrlValueSerializer {
    value: String,
}

const PATH_SET: &AsciiSet = &CONTROLS
    .add(b'~')
    .add(b' ')
    .add(b'"')
    .add(b'#')
    .add(b'<')
    .add(b'>')
    .add(b'?')
    .add(b'`')
    .add(b'{')
    .add(b'}');

impl<'a> ser::Serializer for &'a mut UrlValueSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ErrorSerializer;
    type SerializeTuple = ErrorSerializer;
    type SerializeTupleStruct = ErrorSerializer;
    type SerializeTupleVariant = ErrorSerializer;
    type SerializeMap = ErrorSerializer;
    type SerializeStruct = ErrorSerializer;
    type SerializeStructVariant = ErrorSerializer;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        self.value = utf8_percent_encode(if v { "true" } else { "false" });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.value = utf8_percent_encode(buffer.format(v));
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.value = utf8_percent_encode(buffer.format(v));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.value = utf8_percent_encode(buffer.format(v));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.value = utf8_percent_encode(buffer.format(v));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.value = utf8_percent_encode(buffer.format(v));
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.value = utf8_percent_encode(buffer.format(v));
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.value = utf8_percent_encode(buffer.format(v));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        let mut buffer = itoa::Buffer::new();
        self.value = utf8_percent_encode(buffer.format(v));
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        let mut buf = ryu::Buffer::new();
        self.value = utf8_percent_encode(buf.format(v));
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        let mut buf = ryu::Buffer::new();
        self.value = utf8_percent_encode(buf.format(v));
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        let mut buf = [0; 4];
        self.value = utf8_percent_encode(v.encode_utf8(&mut buf));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.value = utf8_percent_encode(v);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.value = percent_encoding::percent_encode(v, PATH_SET).to_string();
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.value = utf8_percent_encode("");
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        self.value = utf8_percent_encode(variant);
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error> {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Err(Error::ValueNotSupported)
    }
}

struct ErrorSerializer;

impl ser::SerializeSeq for ErrorSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::ValueNotSupported)
    }
}

impl ser::SerializeTuple for ErrorSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::ValueNotSupported)
    }
}

impl ser::SerializeTupleStruct for ErrorSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::ValueNotSupported)
    }
}

impl ser::SerializeTupleVariant for ErrorSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::ValueNotSupported)
    }
}

impl ser::SerializeMap for ErrorSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _key: &T) -> Result<(), Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<(), Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::ValueNotSupported)
    }
}

impl ser::SerializeStruct for ErrorSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::ValueNotSupported)
    }
}

impl ser::SerializeStructVariant for ErrorSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error> {
        Err(Error::ValueNotSupported)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Err(Error::ValueNotSupported)
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    Message(String),
    #[error("Top level serializer only supports structs")]
    TopLevel,
    #[error("Invalid endpoint")]
    InvalidEndpoint,
    #[error("Value not supported")]
    ValueNotSupported,
    #[error("Key not found: {0}")]
    KeyNotFound(&'static str),
    #[error("Unfilled field: {0}")]
    UnfilledField(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Self::Message(msg.to_string())
    }
}

fn utf8_percent_encode(input: &str) -> String {
    percent_encoding::utf8_percent_encode(input, PATH_SET).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_endpoint() {
        let endpoint = "/shows/{id}/seasons/{season}/episodes/{episode}";
        let parts = parse_endpoint(endpoint).unwrap();
        assert_eq!(
            parts,
            vec![
                Part::Raw("/shows/"),
                Part::Param(Param::Key("id")),
                Part::Raw("/seasons/"),
                Part::Param(Param::Key("season")),
                Part::Raw("/episodes/"),
                Part::Param(Param::Key("episode")),
            ]
        );

        let endpoint = "/shows/{id}/seasons/{season}/episodes/{episode}/";
        let parts = parse_endpoint(endpoint).unwrap();
        assert_eq!(
            parts,
            vec![
                Part::Raw("/shows/"),
                Part::Param(Param::Key("id")),
                Part::Raw("/seasons/"),
                Part::Param(Param::Key("season")),
                Part::Raw("/episodes/"),
                Part::Param(Param::Key("episode")),
                Part::Raw("/"),
            ]
        );

        assert_eq!(
            parse_endpoint("/shows/{{id}}").unwrap_err(),
            Error::InvalidEndpoint
        );
        assert_eq!(
            parse_endpoint("/shows/{id}}").unwrap_err(),
            Error::InvalidEndpoint
        );
        assert_eq!(
            parse_endpoint("/shows/{id").unwrap_err(),
            Error::InvalidEndpoint
        );
    }

    #[test]
    fn test_construct_url() {
        #[derive(Serialize)]
        struct Params {
            id: i32,
        }
        #[derive(Serialize)]
        struct Query {
            page: i32,
            limit: Option<i32>,
        }

        let base_url = "https://example.com";
        let endpoint = "/shows/{id}";
        let params = Params { id: 1 };
        let query = Query {
            page: 1,
            limit: None,
        };

        let url = construct_url(base_url, endpoint, &params, &query).unwrap();
        assert_eq!(url, "https://example.com/shows/1?page=1");
    }
}
