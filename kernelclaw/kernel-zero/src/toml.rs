//! KernelClaw Zero-Dep - Minimal TOML parser
//! Minimal TOML for policy config - replaces serde_yaml/toml

/// TOML value
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value>),
    Table(Vec<(String, Value)>),
}

impl Value {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Integer(i) => Some(*i),
            _ => None,
        }
    }
    
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    pub fn as_table(&self) -> Option<&Vec<(String, Value)>> {
        match self {
            Value::Table(t) => Some(t),
            _ => None,
        }
    }
    
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.as_table()?.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }
}

/// Parse TOML string
pub fn parse(s: &str) -> Result<Vec<(String, Value)>, String> {
    let mut result = Vec::new();
    let mut lines: Vec<&str> = s.lines().collect();
    let mut i = 0;
    
    while i < lines.len() {
        let line = lines[i].trim();
        
        // Skip comments and empty
        if line.is_empty() || line.starts_with('#') {
            i += 1;
            continue;
        }
        
        // Section header [section]
        if line.starts_with('[') && line.ends_with(']') {
            let section = &line[1..line.len()-1];
            // Skip for now - would need nested tables
            i += 1;
            continue;
        }
        
        // Key = value
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let value = line[eq_pos+1..].trim().to_string();
            result.push((key, parse_value(&value)));
        }
        
        i += 1;
    }
    
    Ok(result)
}

fn parse_value(s: &str) -> Value {
    let s = s.trim();
    
    // String
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        return Value::String(s[1..s.len()-1].to_string());
    }
    
    // Boolean
    if s == "true" {
        return Value::Boolean(true);
    }
    if s == "false" {
        return Value::Boolean(false);
    }
    
    // Integer
    if let Ok(i) = s.parse::<i64>() {
        return Value::Integer(i);
    }
    
    // Float
    if let Ok(f) = s.parse::<f64>() {
        return Value::Float(f);
    }
    
    // Default to string
    Value::String(s.to_string())
}

/// Serialize to TOML string  
pub fn to_string(table: &[(String, Value)]) -> String {
    table.iter()
        .map(|(k, v)| format!("{} = {}", k, value_to_toml(v)))
        .collect::<Vec<_>>()
        .join("\n")
}

fn value_to_toml(value: &Value) -> String {
    match value {
        Value::String(s) => format!("\"{}\"", s),
        Value::Integer(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Boolean(b) => b.to_string(),
        Value::Array(arr) => {
            let items = arr.iter().map(value_to_toml).collect::<Vec<_>>().join(", ");
            format!("[{}]", items)
        }
        Value::Table(t) => {
            let items = t.iter()
                .map(|(k, v)| format!("{} = {}", k, value_to_toml(v)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{{}}}", items)
        }
    }
}

/// Parse policy YAML (now TOML)
pub fn parse_policy(toml_str: &str) -> Result<Vec<(String, Value)>, String> {
    parse(toml_str)
}