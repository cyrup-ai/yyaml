use crate::linked_hash_map::LinkedHashMap;
use std::hash::{Hash, Hasher};

/// The YAML node representation, mirroring the original design:
/// - `Real` is an f64 stored as string (lazy parse).
/// - `Integer` is i64.
/// - `String` is an owned string.
/// - `Boolean` is bool.
/// - `Array` is a vector of `Yaml`.
/// - `Hash` is an insertion-order map (using `linked_hash_map` logic).
/// - `Alias` for referencing an anchor.
/// - `Null` represents explicit YAML null.
/// - `BadValue` is returned for invalid indexing or out-of-range lookups.
#[derive(Clone, PartialEq, PartialOrd, Debug, Eq, Ord)]
pub enum Yaml {
    Real(String),
    Integer(i64),
    String(String),
    Boolean(bool),
    Array(Vec<Yaml>),
    Hash(LinkedHashMap<Yaml, Yaml>),
    Alias(usize),
    Null,
    BadValue,
}

impl Hash for Yaml {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Yaml::Real(s) => {
                0.hash(state);
                s.hash(state);
            }
            Yaml::Integer(i) => {
                1.hash(state);
                i.hash(state);
            }
            Yaml::String(s) => {
                2.hash(state);
                s.hash(state);
            }
            Yaml::Boolean(b) => {
                3.hash(state);
                b.hash(state);
            }
            Yaml::Array(a) => {
                4.hash(state);
                a.hash(state);
            }
            Yaml::Hash(h) => {
                5.hash(state);
                h.hash(state);
            }
            Yaml::Alias(i) => {
                6.hash(state);
                i.hash(state);
            }
            Yaml::Null => {
                7.hash(state);
            }
            Yaml::BadValue => {
                8.hash(state);
            }
        }
    }
}

/// This `BadValue` is used if we do `doc["unknown"]`, so indexing is graceful.
static BAD_VALUE: Yaml = Yaml::BadValue;

/// Accessors for Yaml
impl Yaml {
    pub fn as_bool(&self) -> Option<bool> {
        match *self {
            Yaml::Boolean(b) => Some(b),
            _ => None,
        }
    }
    
    pub fn as_i64(&self) -> Option<i64> {
        match *self {
            Yaml::Integer(i) => Some(i),
            _ => None,
        }
    }
    
    pub fn as_f64(&self) -> Option<f64> {
        match *self {
            Yaml::Real(ref s) => parse_f64(s),
            _ => None,
        }
    }
    
    pub fn as_str(&self) -> Option<&str> {
        match *self {
            Yaml::String(ref s) => Some(s),
            _ => None,
        }
    }
    
    pub fn as_vec(&self) -> Option<&[Yaml]> {
        match *self {
            Yaml::Array(ref v) => Some(v),
            _ => None,
        }
    }
    
    pub fn as_hash(&self) -> Option<&LinkedHashMap<Yaml, Yaml>> {
        match *self {
            Yaml::Hash(ref h) => Some(h),
            _ => None,
        }
    }
    
    pub fn is_null(&self) -> bool {
        matches!(*self, Yaml::Null)
    }
    
    pub fn is_badvalue(&self) -> bool {
        matches!(*self, Yaml::BadValue)
    }
    
    /// Parse a string into a Yaml value with automatic type detection
    pub fn from_str(v: &str) -> Yaml {
        if v.starts_with("0x") {
            if let Ok(i) = i64::from_str_radix(&v[2..], 16) {
                return Yaml::Integer(i);
            }
        }
        if v.starts_with("0o") {
            if let Ok(i) = i64::from_str_radix(&v[2..], 8) {
                return Yaml::Integer(i);
            }
        }
        if v.starts_with('+') {
            if let Ok(i) = v[1..].parse::<i64>() {
                return Yaml::Integer(i);
            }
        }
        match v {
            "~" | "null" => Yaml::Null,
            "true" => Yaml::Boolean(true),
            "false" => Yaml::Boolean(false),
            _ if v.parse::<i64>().is_ok() => Yaml::Integer(v.parse::<i64>().unwrap()),
            _ if parse_f64(v).is_some() => Yaml::Real(v.into()),
            _ => Yaml::String(v.into()),
        }
    }
}

/// Convert string to float (including .inf, .nan).
fn parse_f64(v: &str) -> Option<f64> {
    match v {
        ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => Some(f64::INFINITY),
        "-.inf" | "-.Inf" | "-.INF" => Some(f64::NEG_INFINITY),
        ".nan" | ".NaN" | ".NAN" => Some(f64::NAN),
        _ => v.parse::<f64>().ok(),
    }
}

/// Indexing by &str
impl std::ops::Index<&str> for Yaml {
    type Output = Yaml;
    fn index(&self, idx: &str) -> &Yaml {
        let key = Yaml::String(idx.to_owned());
        match self.as_hash() {
            Some(h) => h.get(&key).unwrap_or(&BAD_VALUE),
            None => &BAD_VALUE,
        }
    }
}

/// Indexing by usize
impl std::ops::Index<usize> for Yaml {
    type Output = Yaml;
    fn index(&self, idx: usize) -> &Yaml {
        if let Some(v) = self.as_vec() {
            v.get(idx).unwrap_or(&BAD_VALUE)
        } else if let Some(h) = self.as_hash() {
            let key = Yaml::Integer(idx as i64);
            h.get(&key).unwrap_or(&BAD_VALUE)
        } else {
            &BAD_VALUE
        }
    }
}