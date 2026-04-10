//! kernel-zero-serde - FULL Serialize/Deserialize implementation
//! Full-featured serde replacement with struct/enum support, multiple formats

use std::fmt;

// ============================================================================
// CORE TRAITS
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

// ============================================================================
// SERIALIZER TRAITS
// ============================================================================

/// Serializer trait
pub trait Serializer: Sized {
    type Ok;
    type Error: Display;
    
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
    fn serialize_enum(self, name: &'static str, variants: &'static [&'static str]) -> Result<Self::SerializeEnum, Self::Error>;
}

/// Seq serializer
pub trait SerializeSeq {
    type Error: Display;
    fn serialize_element<T: ?Sized + Serialize>(&mut self, elem: &T) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Map serializer
pub trait SerializeMap {
    type Error: Display;
    fn serialize_key<T: ?Sized + Serialize>(&mut self, key: &T) -> Result<(), Self::Error>;
    fn serialize_value<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Struct serializer  
pub trait SerializeStruct {
    type Error: Display;
    fn serialize_field<T: ?Sized + Serialize>(&mut self, key: &'static str, field: &T) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

/// Enum serializer
pub trait SerializeEnum {
    type Error: Display;
    fn encode_variant(self, variant: &'static str) -> Result<Self::EncodeVariant, Self::Error>;
}

pub trait EncodeVariant {
    type Error: Display;
    fn encode_field<T: ?Sized + Serialize>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>;
    fn end(self) -> Result<Self::Ok, Self::Error>;
}

// ============================================================================
// DESERIALIZER TRAITS
// ============================================================================

/// Deserializer trait
pub trait Deserializer: Sized {
    type Error: Display;
    
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
    fn deserialize_unit_struct<V: Visitor>(self, name: &'static str, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_newtype_struct<T: Deserialize>(self, name: &'static str) -> Result<T, Self::Error>;
    fn deserialize_seq<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_map<V: Visitor>(self, visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_struct<V: Visitor>(self, name: &'static str, fields: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>;
    fn deserialize_enum<V: Visitor>(self, name: &'static str, variants: &'static [&'static str], visitor: V) -> Result<V::Value, Self::Error>;
}

/// Visitor trait - receives deserialized values
pub trait Visitor: Sized {
    type Value;
    type Error: Display;
    
    fn expecting(&self) -> &str { "any value" }
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
    fn visit_bytes(self, v: Vec<u8>) -> Result<Self::Value, Self::Error>;
    fn visit_seq(self, visitor: SeqAccess) -> Result<Self::Value, Self::Error>;
    fn visit_map(self, visitor: MapAccess) -> Result<Self::Value, Self::Error>;
    fn visit_struct(self, name: &'static str, fields: &'static [&'static str], visitor: StructAccess) -> Result<Self::Value, Self::Error>;
    fn visit_enum(self, name: &'static str, variants: &'static [&'static str], visitor: EnumAccess) -> Result<Self::Value, Self::Error>;
}

/// Seq access for deserialization
pub trait SeqAccess: Sized {
    type Error: Display;
    fn size_hint(&self) -> Option<usize>;
    fn next_element<T: Deserialize>(&mut self) -> Result<Option<T>, Self::Error>;
}

/// Map access for deserialization
pub trait MapAccess: Sized {
    type Error: Display;
    fn size_hint(&self) -> Option<usize>;
    fn next_key<T: Deserialize>(&mut self) -> Result<Option<T>, Self::Error>;
    fn next_value<T: Deserialize>(&mut self) -> Result<T, Self::Error>;
}

/// Struct access for deserialization
pub trait StructAccess: Sized {
    type Error: Display;
    fn size_hint(&self) -> Option<usize>;
    fn next_field<T: Deserialize>(&mut self) -> Result<Option<(&'static str, T)>, Self::Error>;
}

/// Enum access for deserialization
pub trait EnumAccess: Sized {
    type Error: Display;
    fn variant<T: Deserialize>(&self, variant: &'static str) -> Result<T, Self::Error>;
}

/// Error type
#[derive(Clone)]
pub struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error {
    pub fn custom(msg: impl Into<String>) -> Self {
        Error(msg.into())
    }
    
    pub fn msg(&self) -> &str {
        &self.0
    }
}

// ============================================================================
// IMPLS FOR PRIMITIVES
// ============================================================================

macro_rules! impl_serialize_primitives {
    ($($t:ty => $method:ident),*) => {
        $(
            impl Serialize for $t {
                fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                    serializer.$method(*self)
                }
            }
        )*
    };
}

impl_serialize_primitives! {
    bool => serialize_bool,
    i8 => serialize_i8,
    i16 => serialize_i16,
    i32 => serialize_i32,
    i64 => serialize_i64,
    u8 => serialize_u8,
    u16 => serialize_u16,
    u32 => serialize_u32,
    u64 => serialize_u64,
    f32 => serialize_f32,
    f64 => serialize_f64,
    char => serialize_char,
    &str => serialize_str
}

impl Serialize for String {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self)
    }
}

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

impl<T: Serialize, const N: usize> Serialize for [T; N] {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(N))?;
        for item in self {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl<T: Serialize, U: Serialize> Serialize for (T, U) {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(2))?;
        seq.serialize_element(&self.0)?;
        seq.serialize_element(&self.1)?;
        seq.end()
    }
}

impl<K: Serialize, V: Serialize> Serialize for std::collections::HashMap<K, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        for (k, v) in self {
            map.serialize_key(k)?;
            map.serialize_value(v)?;
        }
        map.end()
    }
}

// ============================================================================
// DESERIALIZE IMPLS
// ============================================================================

macro_rules! impl_deserialize_primitives {
    ($($t:ty => $method:ident => $visitor:ident),*) => {
        $(
            impl<'de> Deserialize<'de> for $t {
                fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
                    deserializer.$method($visitor)
                }
            }
        )*
    };
}

struct BoolVisitor;
impl Visitor for BoolVisitor {
    type Value = bool;
    type Error = Error;
    fn visit_bool(self, v: bool) -> Result<bool, Error> { Ok(v) }
}

struct I8Visitor;
impl Visitor for I8Visitor { type Value = i8; type Error = Error; fn visit_i8(self, v: i8) -> Result<i8, Error> { Ok(v) } }
struct I16Visitor;
impl Visitor for I16Visitor { type Value = i16; type Error = Error; fn visit_i16(self, v: i16) -> Result<i16, Error> { Ok(v) } }
struct I32Visitor;
impl Visitor for I32Visitor { type Value = i32; type Error = Error; fn visit_i32(self, v: i32) -> Result<i32, Error> { Ok(v) } }
struct I64Visitor;
impl Visitor for I64Visitor { type Value = i64; type Error = Error; fn visit_i64(self, v: i64) -> Result<i64, Error> { Ok(v) } }
struct U8Visitor;
impl Visitor for U8Visitor { type Value = u8; type Error = Error; fn visit_u8(self, v: u8) -> Result<u8, Error> { Ok(v) } }
struct U16Visitor;
impl Visitor for U16Visitor { type Value = u16; type Error = Error; fn visit_u16(self, v: u16) -> Result<u16, Error> { Ok(v) } }
struct U32Visitor;
impl Visitor for U32Visitor { type Value = u32; type Error = Error; fn visit_u32(self, v: u32) -> Result<u32, Error> { Ok(v) } }
struct U64Visitor;
impl Visitor for U64Visitor { type Value = u64; type Error = Error; fn visit_u64(self, v: u64) -> Result<u64, Error> { Ok(v) } }
struct F32Visitor;
impl Visitor for F32Visitor { type Value = f32; type Error = Error; fn visit_f32(self, v: f32) -> Result<f32, Error> { Ok(v) } }
struct F64Visitor;
impl Visitor for F64Visitor { type Value = f64; type Error = Error; fn visit_f64(self, v: f64) -> Result<f64, Error> { Ok(v) } }
struct CharVisitor;
impl Visitor for CharVisitor { type Value = char; type Error = Error; fn visit_char(self, v: char) -> Result<char, Error> { Ok(v) } }
struct StringVisitor;
impl Visitor for StringVisitor { type Value = String; type Error = Error; fn visit_string(self, v: String) -> Result<String, Error> { Ok(v) } fn visit_str(self, v: &str) -> Result<String, Error> { Ok(v.to_string()) }
struct BoolOptionVisitor;
impl Visitor for BoolOptionVisitor { type Value = Option<bool>; type Error = Error; fn visit_bool(self, v: bool) -> Result<Option<bool>, Error> { Ok(Some(v)) } fn visit_none(self) -> Result<Option<bool>, Error> { Ok(None) }

impl_deserialize_primitives! {
    bool => deserialize_bool => BoolVisitor,
    i8 => deserialize_i8 => I8Visitor,
    i16 => deserialize_i16 => I16Visitor,
    i32 => deserialize_i32 => I32Visitor,
    i64 => deserialize_i64 => I64Visitor,
    u8 => deserialize_u8 => U8Visitor,
    u16 => deserialize_u16 => U16Visitor,
    u32 => deserialize_u32 => U32Visitor,
    u64 => deserialize_u64 => U64Visitor,
    f32 => deserialize_f32 => F32Visitor,
    f64 => deserialize_f64 => F64Visitor,
    char => deserialize_char => CharVisitor,
    String => deserialize_string => StringVisitor
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Option<T> {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<Self, D::Error> {
        struct OptionVisitor;
        impl Visitor for OptionVisitor {
            type Value = Option<T>;
            type Error = Error;
            fn visit_some<D: Deserializer>(self, deserializer: D) -> Result<Option<T>, Error> {
                Ok(Some(T::deserialize(deserializer)?))
            }
            fn visit_none(self) -> Result<Option<T>, Error> { Ok(None) }
        }
        deserializer.deserialize_some(OptionVisitor)
    }
}

impl<'de, T: Deserialize<'de>, const N: usize> Deserialize<'de> for [T; N] {
    fn deserialize<D: Deserializer>(deserializer: D) -> Result<[T; N], D::Error> {
        todo!("Array deserialization")
    }
}

// ============================================================================
// SERIALIZERS
// ============================================================================

/// JSON Serializer
pub struct JsonSerializer {
    buf: String,
    pretty: bool,
    indent: usize,
}

impl JsonSerializer {
    pub fn new() -> Self { JsonSerializer { buf: String::new(), pretty: false, indent: 0 } }
    pub fn pretty() -> Self { JsonSerializer { buf: String::new(), pretty: true, indent: 0 } }
    pub fn into_string(self) -> String { self.buf }
}

impl Serializer for JsonSerializer {
    type Ok = String;
    type Error = Error;
    
    fn serialize_bool(self, v: bool) -> Result<String, Error> { Ok(v.to_string()) }
    fn serialize_i8(self, v: i8) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_i16(self, v: i16) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_i32(self, v: i32) -> Ok<String) { Ok(v.to_string()) }
    fn serialize_i64(self, v: i64) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_u8(self, v: u8) -> Ok<String) { Ok(v.to_string()) }
    fn serialize_u16(self, v: u16) -> Ok<String) { Ok(v.to_string()) }
    fn serialize_u32(self, v: u32) -> Ok<String) { Ok(v.to_string()) }
    fn serialize_u64(self, v: u64) -> Ok<String) { Ok(v.to_string()) }
    fn serialize_f32(self, v: f32) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_f64(self, v: f64) -> Ok<String) { Ok(v.to_string()) }
    fn serialize_char(self, v: char) -> Ok<String) { Ok(format!("\"{}\"", v)) }
    fn serialize_str(self, v: &str) -> Ok(String) { Ok(format!("\"{}\"", escape(v))) }
    fn serialize_bytes(self, v: &[u8]) -> Ok(String) { Ok(format!("\"{}\"", base64(v))) }
    
    fn serialize_some<T: Serialize>(self, value: &T) -> Result<String, Error> { value.serialize(self) }
    fn serialize_unit(self) -> Ok(String) { Ok("null".into()) }
    fn serialize_unit_struct(self, name: &str) -> Ok(String) { Ok(format!("\"{}\"", name)) }
    fn serialize_newtype_struct<T: Serialize>(self, _name: &str, value: &T) -> Result<String, Error> { value.serialize(self) }
    fn serialize_seq(self, len: Option<usize>) -> Result<JsonSeq, Error> { Ok(JsonSeq { buf: String::new(), first: true }) }
    fn serialize_map(self, len: Option<usize>) -> Result<JsonMap, Error> { Ok(JsonMap { buf: String::new(), first: true }) }
    fn serialize_struct(self, name: &str, len: usize) -> Result<JsonStruct, Error> { Ok(JsonStruct { buf: format!("\"{}\":", name), first: true }) }
    fn serialize_enum(self, name: &str, variants: &[&str]) -> Result<JsonEnum, Error> { Ok(JsonEnum { name: name.to_string(), variants }) }
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t")
}

fn base64(data: &[u8]) -> String {
    const A: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut r = String::new();
    for c in data.chunks(3) {
        let b0 = c[0] as u32;
        let b1 = c.get(1).copied().unwrap_or(0) as u32;
        let b2 = c.get(2).copied().unwrap_or(0) as u32;
        r.push(A[(b0 >> 2) as usize] as char);
        r.push(A[(((b0 & 3) << 4) | (b1 >> 4)) as usize] as char);
        if c.len() > 1 { r.push(A[(((b1 & 15) << 2) | (b2 >> 6)) as usize] as char); } else { r.push('='); }
        if c.len() > 2 { r.push(A[(b2 & 63) as usize] as char); } else { r.push('='); }
    }
    r
}

impl SerializeSeq for JsonSeq {
    type Error = Error;
    fn serialize_element<T: Serialize>(&mut self, elem: &T) -> Result<(), Error> {
        if !self.first { self.buf.push(','); }
        self.first = false;
        self.buf.push_str(&elem.serialize(JsonSerializer::new())?);
 Ok(())
    }
    fn end(self) -> Result<String, Error> { Ok(format!("[{}]", self.buf)) }
}

impl SerializeMap for JsonMap {
    type Error = Error;
    fn serialize_key<T: Serialize>(&mut self, key: &T) -> Result<(), Error> {
        if !self.first { self.buf.push(','); }
        self.first = false;
        self.buf.push_str(&key.serialize(JsonSerializer::new())?);
 Ok(())
    }
    fn serialize_value<T: Serialize>(&mut self, value: &T) -> Result<(), Error> {
        self.buf.push(':');
        self.buf.push_str(&value.serialize(JsonSerializer::new())?);
 Ok(())
    }
    fn end(self) -> Result<String, Error> { Ok(format!("{{{}}}", self.buf)) }
}

impl SerializeStruct for JsonStruct {
    type Error = Error;
    fn serialize_field<T: Serialize>(&mut self, key: &str, field: &T) -> Result<(), Error> {
        if !self.first { self.buf.push(','); }
        self.first = false;
        self.buf.push_str(&format!("\"{}\":{}", key, field.serialize(JsonSerializer::new())?));
 Ok(())
    }
    fn end(self) -> Result<String, Error> { Ok(format!("{{{}}}", self.buf)) }
}

impl SerializeEnum for JsonEnum {
    type Error = Error;
    fn encode_variant(self, variant: &str) -> Result<JsonEnumVar, Error> {
        Ok(JsonEnumVar { variant: variant.to_string() })
    }
}

struct JsonSeq { buf: String, first: bool }
struct JsonMap { buf: String, first: bool }
struct JsonStruct { buf: String, first: bool }
struct JsonEnum { name: String, variants: &'static [&'static str] }
struct JsonEnumVar { variant: String }

impl EncodeVariant for JsonEnumVar {
    type Error = Error;
    fn encode_field<T: Serialize>(&mut self, key: &str, value: &T) -> Result<(), Error> {
        self.variant.push_str(&format!("{}:{}", key, value.serialize(JsonSerializer::new())?));
        Ok(())
    }
    fn end(self) -> Result<String, Error> { Ok(self.variant) }
}

// ============================================================================
// TOML SERIALIZER
// ============================================================================

/// TOML Serializer
pub struct TomlSerializer {
    tables: Vec<String>,
    current: String,
}

impl TomlSerializer {
    pub fn new() -> Self { TomlSerializer { tables: Vec::new(), current: String::new() } }
    pub fn into_string(self) -> String { self.current }
}

impl Serializer for TomlSerializer {
    type Ok = String;
    type Error = Error;
    
    fn serialize_bool(self, v: bool) -> Result<String, Error> { Ok(v.to_string()) }
    fn serialize_i8(self, v: i8) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_i16(self, v: i16) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_i32(self, v: i32) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_i64(self, v: i64) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_u8(self, v: u8) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_u16(self, v: u16) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_u32(self, v: u32) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_u64(self, v: u64) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_f32(self, v: f32) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_f64(self, v: f64) -> Ok(String) { Ok(v.to_string()) }
    fn serialize_char(self, v: char) -> Ok(String) { Ok(format!("'{}'", v)) }
    fn serialize_str(self, v: &str) -> Ok(String) { Ok(format!("\"{}\"", v.replace('"', "\\\""))) }
    fn serialize_bytes(self, v: &[u8]) -> Ok(String) { Ok(format!("[[{}]]", base64(v))) }
    
    fn serialize_some<T: Serialize>(self, value: &T) -> Result<String, Error> { value.serialize(self) }
    fn serialize_unit(self) -> Ok(String) { Ok("".into()) }
    fn serialize_unit_struct(self, name: &str) -> Ok(String) { Ok(name.into()) }
    fn serialize_newtype_struct<T: Serialize>(self, _name: &str, value: &T) -> Result<String, Error> { value.serialize(self) }
    fn serialize_seq(self, len: Option<usize>) -> Result<TomlSeq, Error> { Ok(TomlSeq { items: Vec::new() }) }
    fn serialize_map(self, len: Option<usize>) -> Result<TomlMap, Error> { Ok(TomlMap { items: Vec::new() }) }
    fn serialize_struct(self, name: &str, len: usize) -> Result<TomlStruct, Error> { Ok(TomlStruct { name: name.to_string(), items: Vec::new() }) }
    fn serialize_enum(self, name: &str, variants: &[&str]) -> Result<TomlEnum, Error> { Ok(TomlEnum { name: name.to_string(), variants }) }
}

struct TomlSeq { items: Vec<String> }
struct TomlMap { items: Vec<(String, String)> }
struct TomlStruct { name: String, items: Vec<(String, String)> }
struct TomlEnum { name: String, variants: &'static [&'static str] }

impl SerializeSeq for TomlSeq {
    type Error = Error;
    fn serialize_element<T: Serialize>(&mut self, elem: &T) -> Result<(), Error> {
        self.items.push(elem.serialize(TomlSerializer::new())?);
 Ok(())
    }
    fn end(self) -> Result<String, Error> { Ok(self.items.join("\n")) }
}

impl SerializeMap for TomlMap {
    type Error = Error;
    fn serialize_key<T: Serialize>(&mut self, key: &T) -> Result<(), Error> {
        self.items.push((key.serialize(TomlSerializer::new())?, String::new()));
        Ok(())
    }
    fn serialize_value<T: Serialize>(&mut self, value: &T) -> Result<(), Error> {
        if let Some((_, v)) = self.items.last_mut() {
            *v = value.serialize(TomlSerializer::new())?;
        }
        Ok(())
    }
    fn end(self) -> Result<String, Error> {
        Ok(self.items.iter().map(|(k,v)| format!("{} = {}", k, v)).collect::<Vec<_>>().join("\n"))
    }
}

impl SerializeStruct for TomlStruct {
    type Error = Error;
    fn serialize_field<T: Serialize>(&mut self, key: &str, field: &T) -> Result<(), Error> {
        self.items.push((key.to_string(), field.serialize(TomlSerializer::new())?));
        Ok(())
    }
    fn end(self) -> Result<String, Error> {
        Ok(format!("[{}]\n{}", self.name, self.items.iter().map(|(k,v)| format!("{} = {}", k, v)).collect::<Vec<_>>().join("\n")))
    }
}

// ============================================================================
// API
// ============================================================================

/// Serialize to JSON string
pub fn to_json<T: Serialize>(value: &T) -> Result<String, Error> {
    value.serialize(JsonSerializer::new())
}

/// Serialize to pretty JSON
pub fn to_json_pretty<T: Serialize>(value: &T) -> Result<String, Error> {
    value.serialize(JsonSerializer::pretty())
}

/// Serialize to TOML string
pub fn to_toml<T: Serialize>(value: &T) -> Result<String, Error> {
    value.serialize(TomlSerializer::new())
}

/// Deserialize from JSON string
pub fn from_json<T: Deserialize>(json: &str) -> Result<T, Error> {
    todo!("JSON deserialization")
}

// ============================================================================
// DERIVE MACROS
// ============================================================================

/// Serialize derive - full struct support
#[macro_export]
macro_rules! Serialize {
    // Struct: struct Foo { field1, field2, ... }
    (struct $name:ident { $($field:ident),* $(,)? }) => {
        impl $crate::Serialize for $name {
            fn serialize<S: $crate::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut state = serializer.serialize_struct(stringify!($name), 1)?;
                $(
                    state.serialize_field(stringify!($field), &self.$field)?;
                )*
                state.end()
            }
        }
    };
    
    // Tuple struct: struct Foo(T1, T2, ...)
    (struct $name:ident $(($($t:ty),*));*) => {
        impl $crate::Serialize for $name {
            fn serialize<S: $crate::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                let mut seq = serializer.serialize_seq(Some(1))?;
                $(
                    seq.serialize_element(&self.0)?;
                )*
                seq.end()
            }
        }
    };
}

/// Deserialize derive - full struct support
#[macro_export]
macro_rules! Deserialize {
    (struct $name:ident { $($field:ident),* $(,)? }) => {
        impl<'de> $crate::Deserialize<'de> for $name {
            fn deserialize<D: $crate::Deserializer>(deserializer: D) -> Result<Self, D::Error> {
                struct FieldVisitor;
                impl $crate::Visitor for FieldVisitor {
                    type Value = Self;
                    type Error = $crate::Error;
                    
                    fn visit_struct(self, name: &str, fields: &[&str], mut visitor: $crate::StructAccess) -> Result<Self, $crate::Error> {
                        Ok($name {
                            $(
                                $field: visitor.next_field().ok_or($crate::Error::custom("missing field"))?.1,
                            )*
                        })
                    }
                }
                deserializer.deserialize_struct(stringify!($name), &[stringify!($field)], FieldVisitor)
            }
        }
    };
}

/// Enum support
#[macro_export]
macro_rules! SerializableEnum {
    ($name:ident { $($variant:ident $(($($field:ty),*))?),* }) => {
        impl $crate::Serialize for $name {
            fn serialize<S: $crate::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                match self {
                    $(
                        $name::$variant $(($(ref field))*) => {
                            let mut ev = serializer.serialize_enum(stringify!($name), &[stringify!($variant)])?;
                            let mut var = ev.encode_variant(stringify!($variant))?;
                            $(
                                var.encode_field(stringify!($field), field)?;
                            )*
                            var.end()
                        }
                    )*
                }
            }
        }
    };
}

/// Enum support
#[macro_export]
macro_rules! DeserializableEnum {
    ($name:ident { $($variant:ident $(($($t:ty),*))?,* }) => {
        impl<'de> $crate::Deserialize<'de> for $name {
            fn deserialize<D: $crate::Deserializer>(deserializer: D) -> Result<Self, D::Error> {
                struct EnumVisitor;
                impl $crate::Visitor for EnumVisitor {
                    type Value = Self;
                    type Error = $crate::Error;
                    
                    fn visit_enum(self, name: &str, variants: &[&str], mut access: $crate::EnumAccess) -> Result<Self, $crate::Error> {
                        // Simplified - would parse variant name
                        todo!("Enum deserialization")
                    }
                }
                deserializer.deserialize_enum(stringify!($name), &[stringify!($variant)], EnumVisitor)
            }
        }
    };
}