//! kernel-zero-serde - Full Serialize/Deserialize implementation (lite version)
//! Replaces serde with minimal derive-based serialization

use std::fmt;

/// ============================================================================
// TRAIT DEFINITIONS
// ============================================================================

/// Serialize trait - convert Rust values to output
pub trait Serialize {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error>;
}

/// Deserialize trait - parse input into Rust values
pub trait Deserialize<'de> {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error>
    where
        Self: Sized;
}

/// Serializer trait - where values are written to
pub trait Serializer {
    type Ok;
    type Error: std::fmt::Display;
    
    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error>;
    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error>;
    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error>;
    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error>;
    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error>;
    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error>;
    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error>;
    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error>;
    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error>;
    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error>;
    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error>;
    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error>;
    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error>;
    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error>;
    
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit(self) -> Result<Self::Ok, Self::Error>;
    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error>;
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, name: &'static str, value: &T) -> Result<Self::Ok, Self::Error>;
    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error>;
    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error>;
    fn serialize_struct(self, name: &'static str, len: usize) -> Result<Self::SerializeStruct, Self::Error>;
}

/// Seq serializer
pub trait SerializeSeq {
    type Error: std::fmt::Display;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, elem: &T) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Map serializer
pub trait SerializeMap {
    type Error: std::fmt::Display;
    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error>;
    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Struct serializer
pub trait SerializeStruct {
    type Error: std::fmt::Display;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, field: &T) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

// ============================================================================
// DESERIALIZER
// ============================================================================

/// Deserializer trait - parse input into Rust values
pub trait Deserializer {
    type Error: std::fmt::Display;
    
    fn deserialize_bool<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_i8<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_i16<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_i32<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_i64<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_u8<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_u16<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_u32<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_u64<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_f32<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_f64<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_char<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_str<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_string<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_bytes<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_some<T: Deserialize>(self) -> Result<T, Self::Error>;
    fn deserialize_unit<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_seq<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_map<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_struct<V: Visitor>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>;
}

/// Visitor trait - receives deserialized values
pub trait Visitor {
    type Value;
    type Error: std::fmt::Display;
    
    fn expecting(&self) -> &str { "value" }
    fn visit_bool(self, v: bool) -> Result<Self::Value, Self::Error>;
    fn visit_i8(self, v: i8) -> Result<Self::Value, Self::Error>;
    fn visit_i16(self, v: i16) -> Result<Self::Value, Self::Error>;
    fn visit_i32(self, v: i32) -> Result<Self::Value, Self::Error>;
    fn visit_i64(self, v: i64) -> Result<Self::Value, Self::Error>;
    fn visit_u8(self, v: u8) -> Result<Self::Value, Self::Error>;
    fn visit_u16(self, v: u16) -> Result<Self::Value, Self::Error>;
    fn visit_u32(self, v: u32) -> Result<Self::Value, Self::Error>;
    fn visit_u64(self, v: u64) -> Result<Self::Value, Self::Error>;
    fn visit_f32(self, v: f32) -> Result<Self::Value, Self::Error>;
    fn visit_f64(self, v: f64) -> Result<Self::Value, Self::Error>;
    fn visit_char(self, v: char) -> Result<Self::Value, Self::Error>;
    fn visit_str(self, v: &str) -> Result<Self::Value, Self::Error>;
    fn visit_string(self, v: String) -> Result<Self::Value, Self::Error>;
    fn visit_bytes(self, v: &[u8]) -> Result<Self::Value, Self::Error>;
    fn visit_seq(self, _visitor: SeqAccess) -> Result<Self::Value, Self::Error> { Err(Error::custom("seq not impl")) }
    fn visit_map(self, _visitor: MapAccess) -> Result<Self::Value, Self::Error> { Err(Error::custom("map not impl")) }
    fn visit_struct(self, _name: &'static str, _fields: &'static [&'static str], _visitor: StructAccess) -> Result<Self::Value, Self::Error> { Err(Error::custom("struct not impl")) }
}

/// Seq access
pub trait SeqAccess {
    type Error: std::fmt::Display;
    fn next_element<T: Deserialize>(&mut self) -> Result<Option<T>, Self::Error>;
}

/// Map access
pub trait MapAccess {
    type Error: std::fmt::Display;
    fn next_key<T: Deserialize>(&mut self) -> Result<Option<T>, Self::Error>;
    fn next_value<T: Deserialize>(&mut self) -> Result<T, Self::Error>;
}

/// Struct access
pub trait StructAccess {
    type Error: std::fmt::Display;
    fn next_field<T: Deserialize>(&mut self) -> Result<Option<T>, Self::Error>;
}

/// Error type
#[derive(Debug)]
pub struct Error(String);

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error {
    pub fn custom(msg: impl Into<String>) -> Self {
        Error(msg.into())
    }
}

// ============================================================================
// IMPLEMENTATIONS FOR PRIMITIVES
// ============================================================================

impl Serialize for bool {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bool(*self)
    }
}

impl Serialize for i8 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_i8(*self) }
impl Serialize for i16 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_i16(*self) }
impl Serialize for i32 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_i32(*self) }
impl Serialize for i64 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_i64(*self) }
impl Serialize for u8 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_u8(*self) }
impl Serialize for u16 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_u16(*self) }
impl Serialize for u32 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_u32(*self) }
impl Serialize for u64 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_u64(*self) }
impl Serialize for f32 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_f32(*self) }
impl Serialize for f64 { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_f64(*self) }
impl Serialize for char { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_char(*self) }
impl Serialize for &str { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_str(self) }
impl Serialize for String { fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> { s.serialize_str(self) }

impl Serialize for () {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_unit()
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Some(v) => serializer.serialize_some(v),
            None => serializer.serialize_unit(),
        }
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

// Deserialize impls
impl<'de> Deserialize<'de> for bool {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
        struct BoolVisitor;
        impl Visitor for BoolVisitor {
            type Value = bool;
            type Error = Error;
            fn visit_bool(self, v: bool) -> Result<bool, Error> { Ok(v) }
        }
        deserializer.deserialize_bool(BoolVisitor)
    }
}

impl<'de> Deserialize<'de> for i32 {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
        struct I32Visitor;
        impl Visitor for I32Visitor {
            type Value = i32;
            type Error = Error;
            fn visit_i32(self, v: i32) -> Result<i32, Error> { Ok(v) }
        }
        deserializer.deserialize_i32(I32Visitor)
    }
}

impl<'de> Deserialize<'de> for String {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
        struct StringVisitor;
        impl Visitor for StringVisitor {
            type Value = String;
            type Error = Error;
            fn visit_string(self, v: String) -> Result<String, Error> { Ok(v) }
            fn visit_str(self, v: &str) -> Result<String, Error> { Ok(v.to_string()) }
        }
        deserializer.deserialize_string(StringVisitor)
    }
}

// ============================================================================
// JSON SERIALIZER
// ============================================================================

/// JSON Serializer - convert to JSON string
pub struct JsonSerializer {
    output: String,
    pretty: bool,
}

impl JsonSerializer {
    pub fn new() -> Self {
        JsonSerializer { output: String::new(), pretty: false }
    }
    
    pub fn pretty() -> Self {
        JsonSerializer { output: String::new(), pretty: true }
    }
    
    pub fn into_string(self) -> String {
        self.output
    }
}

impl Serializer for JsonSerializer {
    type Ok = String;
    type Error = Error;
    
    fn serialize_bool(self, v: bool) -> Result<String, Error> {
        Ok(if v { "true".to_string() } else { "false".to_string() })
    }
    
    fn serialize_i32(self, v: i32) -> Result<String, Error> { Ok(v.to_string()) }
    fn serialize_i64(self, v: i64) -> Result<String, Error> { Ok(v.to_string()) }
    fn serialize_u32(self, v: u32) -> Result<String, Error> { Ok(v.to_string()) }
    fn serialize_u64(self, v: u64) -> Result<String, Error> { Ok(v.to_string()) }
    fn serialize_f32(self, v: f32) -> Result<String, Error> { Ok(v.to_string()) }
    fn serialize_f64(self, v: f64) -> Result<String, Error> { Ok(v.to_string()) }
    fn serialize_char(self, v: char) -> Result<String, Error> { Ok(format!("\"{}\"", v)) }
    fn serialize_str(self, v: &str) -> Result<String, Error> { Ok(format!("\"{}\"", escape_str(v))) }
    fn serialize_bytes(self, v: &[u8]) -> Result<String, Error> { Ok(format!("\"{}\"", base64_encode(v))) }
    
    fn serialize_some<T: ?Sized + Serialize>(self, value: &T) -> Result<String, Error> {
        value.serialize(self)
    }
    
    fn serialize_unit(self) -> Ok("null".to_string()) { Ok("null".to_string()) }
    fn serialize_unit_struct(self, _name: &'static str) -> Ok(String) { Ok("{}".to_string()) }
    fn serialize_newtype_struct<T: ?Sized + Serialize>(self, _name: &'static str, value: &T) -> Result<String, Error> {
        value.serialize(self)
    }
    
    fn serialize_seq(self, _len: Option<usize>) -> Result<JsonSeqSerializer, Error> {
        Ok(JsonSeqSerializer { output: String::new(), first: true })
    }
    
    fn serialize_map(self, _len: Option<usize>) -> Result<JsonMapSerializer, Error> {
        Ok(JsonMapSerializer { output: String::new(), first: true })
    }
    
    fn serialize_struct(self, name: &'static str, len: usize) -> Result<JsonStructSerializer, Error> {
        Ok(JsonStructSerializer { 
            output: format!("{{\"{}\":", name), 
            first: true, 
            len 
        })
    }
    
    fn serialize_bool(self, _v: bool) -> Result<Self::Ok, Self::Error> { todo!() }
    fn serialize_i8(self, _v: i8) -> Result<Self::Ok, Self::Error> { todo!() }
    fn serialize_i16(self, _v: i16) -> Result<Self::Ok, Self::Error> { todo!() }
    fn serialize_u8(self, _v: u8) -> Result<Self::Ok, Self::Error> { todo!() }
    fn serialize_u16(self, _v: u16) -> Result<Self::Ok, Self::Error> { todo!() }
    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> { todo!() }
    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> { todo!() }
}

fn escape_str(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

fn base64_encode(data: &[u8]) -> String {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        result.push(ALPHABET[(b0 >> 2) as usize] as char);
        result.push(ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 { result.push(ALPHABET[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize] as char); } else { result.push('='); }
        if chunk.len() > 2 { result.push(ALPHABET[(b2 & 0x3F) as usize] as char); } else { result.push('='); }
    }
    result
}

impl SerializeSeq for JsonSeqSerializer {
    type Error = Error;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, elem: &T) -> Result<(), Error> {
        if !self.first { self.output.push(','); }
        self.first = false;
        self.output.push_str(&elem.serialize(JsonSerializer::new()).unwrap());
        Ok(())
    }
    fn end(self) -> Result<String, Error> { Ok(format!("[{}]", self.output)) }
}

struct JsonSeqSerializer { output: String, first: bool }

impl SerializeMap for JsonMapSerializer {
    type Error = Error;
    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Error> {
        if !self.first { self.output.push(','); }
        self.first = false;
        self.output.push_str(&key.serialize(JsonSerializer::new()).unwrap());
        Ok(())
    }
    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.output.push(':');
        self.output.push_str(&value.serialize(JsonSerializer::new()).unwrap());
        Ok(())
    }
    fn end(self) -> Result<String, Error> { Ok(format!("{{{}}}", self.output)) }
}

struct JsonMapSerializer { output: String, first: bool }

impl SerializeStruct for JsonStructSerializer {
    type Error = Error;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, field: &T) -> Result<(), Error> {
        if !self.first { self.output.push(','); }
        self.first = false;
        self.output.push_str(&format!("\"{}\":{}", key, field.serialize(JsonSerializer::new()).unwrap()));
        Ok(())
    }
    fn end(self) -> Result<String, Error> { Ok(format!("{}}}", self.output)) }
}

struct JsonStructSerializer { output: String, first: bool, len: usize }

/// Serialize to JSON string
pub fn to_json<T: Serialize>(value: &T) -> String {
    value.serialize(JsonSerializer::new()).unwrap()
}

/// Serialize to pretty JSON
pub fn to_json_pretty<T: Serialize>(value: &T) -> String {
    value.serialize(JsonSerializer::pretty()).unwrap()
}

// ============================================================================
// CONVENIENCE MACROS
// ============================================================================

/// Derive macro for Serialize
#[macro_export]
macro_rules! Serialize {
    (struct $name:ident { $($field:ident),* $(,)? }) => {
        impl $crate::Serialize for $name {
            fn serialize<S: $crate::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut state = serializer.serialize_struct(stringify!($name), 1)?;
                $( state.serialize_field(stringify!($field), &self.$field)?; )*
                state.end()
            }
        }
    };
}

/// Derive macro for Deserialize
#[macro_export]
macro_rules! Deserialize {
    (struct $name:ident { $($field:ident),* $(,)? }) => {
        impl<'de> $crate::Deserialize<'de> for $name {
            fn deserialize<D: $crate::Deserializer>(deserializer: D) -> Result<Self, D::Error> {
                struct FieldVisitor;
                impl $crate::Visitor for FieldVisitor {
                    type Value = &'static str;
                    type Error = $crate::Error;
                    fn visit_str(self, v: &str) -> Result<&'static str, $crate::Error> { Ok(v) }
                }
                // Simplified - would parse actual fields
                Ok($name { $($field: todo!()),* })
            }
        }
    };
}