//! kernel-zero-json - JSON parsing (lite version)
//! Provides basic JSON parse/serialize

use std::fmt;

/// Result type
pub type Result<T> = std::result::Result<T, Error>;

/// Error type
#[derive(Debug, Clone)]
pub struct Error(String);

impl Error {
    pub fn new(msg: impl Into<String>) -> Error {
        Error(msg.into())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// JSON Value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

/// JSON Number (integer or float)
#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

/// Parse JSON string to Value
pub fn parse(s: &str) -> Result<Value> {
    let chars: Vec<char> = s.chars().collect();
    let mut pos = 0;
    parse_value(&chars, &mut pos)
}

fn parse_value(chars: &[char], pos: &mut usize) -> Result<Value> {
    skip_whitespace(chars, pos);
    
    if *pos >= chars.len() {
        return Err(Error::new("unexpected end"));
    }
    
    match chars[*pos] {
        'n' => parse_null(chars, pos),
        't' => parse_true(chars, pos),
        'f' => parse_false(chars, pos),
        '"' => parse_string(chars, pos),
        '[' => parse_array(chars, pos),
        '{' => parse_object(chars, pos),
        '-' | '0'..='9' => parse_number(chars, pos),
        _ => Err(Error::new(format!("unexpected char: {}", chars[*pos]))),
    }
}

fn skip_whitespace(chars: &[char], pos: &mut usize) {
    while *pos < chars.len() && chars[*pos].is_whitespace() {
        *pos += 1;
    }
}

fn parse_null(chars: &[char], pos: &mut usize) -> Result<Value> {
    if chars[*pos..].starts_with(&['n','u','l','l'][..]) {
        *pos += 4;
        Ok(Value::Null)
    } else {
        Err(Error::new("expected null"))
    }
}

fn parse_true(chars: &[char], pos: &mut usize) -> Result<Value> {
    if chars[*pos..].starts_with(&['t','r','u','e'][..]) {
        *pos += 4;
        Ok(Value::Bool(true))
    } else {
        Err(Error::new("expected true"))
    }
}

fn parse_false(chars: &[char], pos: &mut usize) -> Result<Value> {
    if chars[*pos..].starts_with(&['f','a','l','s','e'][..]) {
        *pos += 5;
        Ok(Value::Bool(false))
    } else {
        Err(Error::new("expected false"))
    }
}

fn parse_string(chars: &[char], pos: &mut usize) -> Result<Value> {
    *pos += 1; // skip opening "
    let mut s = String::new();
    
    while *pos < chars.len() && chars[*pos] != '"' {
        if chars[*pos] == '\\' && *pos + 1 < chars.len() {
            *pos += 1;
            match chars[*pos] {
                'n' => s.push('\n'),
                'r' => s.push('\r'),
                't' => s.push('\t'),
                '"' => s.push('"'),
                '\\' => s.push('\\'),
                _ => s.push(chars[*pos]),
            }
        } else {
            s.push(chars[*pos]);
        }
        *pos += 1;
    }
    
    *pos += 1; // skip closing "
    Ok(Value::String(s))
}

fn parse_number(chars: &[char], pos: &mut usize) -> Result<Value> {
    let start = *pos;
    
    // Handle negative
    if chars[*pos] == '-' {
        *pos += 1;
    }
    
    // Digits before decimal
    while *pos < chars.len() && chars[*pos].is_ascii_digit() {
        *pos += 1;
    }
    
    // Decimal part
    if *pos < chars.len() && chars[*pos] == '.' {
        *pos += 1;
        while *pos < chars.len() && chars[*pos].is_ascii_digit() {
            *pos += 1;
        }
    }
    
    // Exponent
    if *pos < chars.len() && (chars[*pos] == 'e' || chars[*pos] == 'E') {
        *pos += 1;
        if *pos < chars.len() && (chars[*pos] == '+' || chars[*pos] == '-') {
            *pos += 1;
        }
        while *pos < chars.len() && chars[*pos].is_ascii_digit() {
            *pos += 1;
        }
    }
    
    let num_str: String = chars[start..*pos].iter().collect();
    
    // Try int first
    if let Ok(i) = num_str.parse::<i64>() {
        return Ok(Value::Number(Number::Int(i)));
    }
    
    // Fall back to float
    match num_str.parse::<f64>() {
        Ok(f) => Ok(Value::Number(Number::Float(f))),
        Err(_) => Err(Error::new(format!("invalid number: {}", num_str))),
    }
}

fn parse_array(chars: &[char], pos: &mut usize) -> Result<Value> {
    *pos += 1; // skip [
    skip_whitespace(chars, pos);
    
    let mut items = Vec::new();
    
    if chars[*pos] == ']' {
        *pos += 1;
        return Ok(Value::Array(items));
    }
    
    loop {
        items.push(parse_value(chars, pos)?);
        skip_whitespace(chars, pos);
        
        if chars[*pos] == ']' {
            *pos += 1;
            break;
        }
        
        if chars[*pos] == ',' {
            *pos += 1;
            skip_whitespace(chars, pos);
        } else {
            return Err(Error::new("expected , or ]"));
        }
    }
    
    Ok(Value::Array(items))
}

fn parse_object(chars: &[char], pos: &mut usize) -> Result<Value> {
    *pos += 1; // skip {
    skip_whitespace(chars, pos);
    
    let mut items = Vec::new();
    
    if chars[*pos] == '}' {
        *pos += 1;
        return Ok(Value::Object(items));
    }
    
    loop {
        skip_whitespace(chars, pos);
        
        // Key
        let key = match parse_value(chars, pos)? {
            Value::String(s) => s,
            _ => return Err(Error::new("expected string key")),
        };
        
        skip_whitespace(chars, pos);
        
        // Colon
        if chars[*pos] == ':' {
            *pos += 1;
        } else {
            return Err(Error::new("expected :"));
        }
        
        // Value
        let value = parse_value(chars, pos)?;
        items.push((key, value));
        
        skip_whitespace(chars, pos);
        
        if chars[*pos] == '}' {
            *pos += 1;
            break;
        }
        
        if chars[*pos] == ',' {
            *pos += 1;
            skip_whitespace(chars, pos);
        } else {
            return Err(Error::new("expected , or }"));
        }
    }
    
    Ok(Value::Object(items))
}

/// Serialize Value to JSON string
pub fn to_string(value: &Value) -> Result<String> {
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Number(n) => match n {
            Number::Int(i) => Ok(i.to_string()),
            Number::Float(f) => Ok(f.to_string()),
        },
        Value::String(s) => Ok(format!("\"{}\"", escape_string(s))),
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter()
                .map(|v| to_string(v))
                .collect::<Result<Vec<_>>>()?;
            Ok(format!("[{}]", items.join(",")))
        }
        Value::Object(obj) => {
            let items: Vec<String> = obj.iter()
                .map(|(k, v)| format!("\"{}\":{}", escape_string(k), to_string(v)?))
                .collect();
            Ok(format!("{{{}}}", items.join(",")))
        }
    }
}

fn escape_string(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

/// Pretty print JSON
pub fn to_string_pretty(value: &Value) -> Result<String> {
    to_string_pretty_indent(value, 0)
}

fn to_string_pretty_indent(value: &Value, indent: usize) -> Result<String> {
    let spaces = |n| " ".repeat(n);
    
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Number(n) => match n {
            Number::Int(i) => Ok(i.to_string()),
            Number::Float(f) => Ok(f.to_string()),
        },
        Value::String(s) => Ok(format!("\"{}\"", escape_string(s))),
        Value::Array(arr) => {
            if arr.is_empty() {
                return Ok("[]".to_string());
            }
            let items: Vec<String> = arr.iter()
                .map(|v| format!("{}", to_string_pretty_indent(v, indent + 2)))
                .collect();
            Ok(format!("[\n{}{}\n{}]", 
                spaces(indent + 2),
                items.join(&format!(",\n{}", spaces(indent + 2))),
                spaces(indent)))
        }
        Value::Object(obj) => {
            if obj.is_empty() {
                return Ok("{}".to_string());
            }
            let items: Vec<String> = obj.iter()
                .map(|(k, v)| format!("\"{}\": {}", escape_string(k), to_string_pretty_indent(v, indent + 2)))
                .collect();
            Ok(format!("{{\n{}{}\n{}}}", 
                spaces(indent + 2),
                items.join(&format!(",\n{}", spaces(indent + 2))),
                spaces(indent)))
        }
    }
}

/// Get value from object by key
pub fn get<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    match value {
        Value::Object(items) => items.iter().find(|(k, _)| k == key).map(|(_, v)| v),
        _ => None,
    }
}

/// Get value from array by index
pub fn get_index(value: &Value, index: usize) -> Option<&Value> {
    match value {
        Value::Array(arr) => arr.get(index),
        _ => None,
    }
}

/// Macro for quick JSON building
#[macro_export]
macro_rules! json {
    (null) => { $crate::Value::Null };
    ($($k:ident : $v:expr),* $(,)?) => {{
        let mut obj = Vec::new();
        $(obj.push((stringify!($k).to_string(), $v));)*
        $crate::Value::Object(obj)
    }};
    ($($v:expr),* $(,)?) => {{
        let mut arr = Vec::new();
        $(arr.push($v);)*
        $crate::Value::Array(arr)
    }};
}