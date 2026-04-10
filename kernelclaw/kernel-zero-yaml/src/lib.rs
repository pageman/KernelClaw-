//! kernel-zero-yaml - YAML parsing (lite version)
//! Provides basic YAML parse/serialize

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

/// YAML Value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    List(Vec<Value>),
    Map(Vec<(String, Value)>),
}

/// YAML Number
#[derive(Debug, Clone, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
}

/// Parse YAML string to Value
pub fn parse(s: &str) -> Result<Value> {
    let lines: Vec<&str> = s.lines().collect();
    parse_yaml_lines(&lines, 0, &mut 0)
}

fn parse_yaml_lines(lines: &[&str], indent: usize, pos: &mut usize) -> Result<Value> {
    if *pos >= lines.len() {
        return Ok(Value::Null);
    }
    
    let mut items = Vec::new();
    let mut map_items = Vec::new();
    
    while *pos < lines.len() {
        let line = lines[*pos];
        
        if line.trim().is_empty() {
            *pos += 1;
            continue;
        }
        
        let line_indent = line.len() - line.trim_start().len();
        
        if line_indent < indent {
            break;
        }
        
        if line.trim().starts_with('#') {
            *pos += 1;
            continue;
        }
        
        if let Some((key, value)) = parse_key_value(line, line_indent) {
            if value.is_none() {
                *pos += 1;
                let child = parse_yaml_lines(lines, line_indent + 2, pos)?;
                map_items.push((key, child));
            } else {
                map_items.push((key, value.unwrap()));
                *pos += 1;
            }
        } else if line.trim().starts_with('-') {
            let item_str = line.trim().trim_start_matches('-').trim();
            if let Ok(v) = parse_yaml_value(item_str) {
                items.push(v);
            }
            *pos += 1;
        } else {
            *pos += 1;
        }
    }
    
    if !map_items.is_empty() {
        Ok(Value::Map(map_items))
    } else if !items.is_empty() {
        Ok(Value::List(items))
    } else {
        Ok(Value::Null)
    }
}

fn parse_key_value(line: &str, _indent: usize) -> Option<(String, Option<Value>)> {
    let trimmed = line.trim();
    if let Some(colon_pos) = trimmed.find(':') {
        let key = trimmed[..colon_pos].to_string();
        let value_str = trimmed[colon_pos + 1..].trim();
        if value_str.is_empty() {
            Some((key, None))
        } else {
            Some((key, Some(parse_yaml_value(value_str).ok()?)))
        }
    } else {
        None
    }
}

fn parse_yaml_value(s: &str) -> Result<Value> {
    let s = s.trim();
    if s == "null" || s == "~" || s.is_empty() {
        return Ok(Value::Null);
    }
    if s == "true" || s == "yes" || s == "on" {
        return Ok(Value::Bool(true));
    }
    if s == "false" || s == "no" || s == "off" {
        return Ok(Value::Bool(false));
    }
    if let Ok(i) = s.parse::<i64>() {
        return Ok(Value::Number(Number::Int(i)));
    }
    if let Ok(f) = s.parse::<f64>() {
        return Ok(Value::Number(Number::Float(f)));
    }
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        return Ok(Value::String(s[1..s.len()-1].to_string()));
    }
    Ok(Value::String(s.to_string()))
}

pub fn to_string(value: &Value) -> Result<String> {
    to_string_indent(value, 0)
}

fn to_string_indent(value: &Value, _indent: usize) -> Result<String> {
    match value {
        Value::Null => Ok("null".to_string()),
        Value::Bool(b) => Ok(b.to_string()),
        Value::Number(n) => match n { Number::Int(i) => Ok(i.to_string()), Number::Float(f) => Ok(f.to_string()) },
        Value::String(s) => if s.contains(':') || s.starts_with(' ') { Ok(format!("\"{}\"", s)) } else { Ok(s.clone()) },
        Value::List(list) => { let items: Vec<String> = list.iter().map(|v| format!("- {}", to_string_indent(v, 0)?)).collect(); Ok(items.join("\n")) }
        Value::Map(map) => { let items: Vec<String> = map.iter().map(|(k,v)| format!("{}: {}", k, to_string_indent(v, 0)?)).collect(); Ok(items.join("\n")) }
    }
}

pub fn to_string_pretty(value: &Value) -> Result<String> { to_string(value) }

pub fn get<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    match value { Value::Map(items) => items.iter().find(|(k,_)| k == key).map(|(_,v)| v), _ => None }
}

pub fn get_index(value: &Value, index: usize) -> Option<&Value> {
    match value { Value::List(list) => list.get(index), _ => None }
}

#[macro_export]
macro_rules! yaml {
    (null) => { $crate::Value::Null };
    ($($k:ident : $v:expr),* $(,)?) => {{ let mut map = Vec::new(); $(map.push((stringify!($k).to_string(), $v));)* $crate::Value::Map(map) }};
    ($($v:expr),*) => {{ let mut list = Vec::new(); $(list.push($v);)* $crate::Value::List(list) }};
}