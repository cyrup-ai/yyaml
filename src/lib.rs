//! A comprehensive YAML 1.2 parser/emitter supporting anchors, aliases,
//! block & flow sequences/mappings, block scalars, tags, multi-doc streams, etc.
//!
//! # Example
//! ```rust
//! let docs = yyaml::YamlLoader::load_from_str("foo: 123").unwrap();
//! let doc = &docs[0];
//! assert_eq!(doc["foo"].as_i64().unwrap(), 123);
//! ```

mod de;
mod emitter;
mod error;
mod events;
pub mod lexer;
mod linked_hash_map;
pub mod parser;

pub mod scanner;
pub mod semantic;
mod ser;
mod value;
mod yaml;

pub use de::*;
pub use emitter::{EmitError, EmitResult, YamlEmitter};
pub use error::{Marker, ScanError};
pub use events::{Event, EventReceiver, MarkedEventReceiver, TEncoding, TScalarStyle, TokenType};
pub use linked_hash_map::LinkedHashMap;
pub use parser::YamlLoader;
pub use ser::*;
pub use value::{Deserializer, Mapping, Number, Sequence, Value, from_value};
pub use yaml::Yaml;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("scan error: {0}")]
    Scan(#[from] ScanError),
    #[error("emit error: {0}")]
    Emit(#[from] EmitError),
    #[error("custom: {0}")]
    Custom(String),
}

impl serde::de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

pub fn to_string<T: serde::Serialize>(value: &T) -> Result<String, Error> {
    let yaml = value.serialize(ser::YamlSerializer::new())?;
    let mut writer = String::new();
    let mut emitter = YamlEmitter::new(&mut writer);
    emitter.dump(&yaml)?;
    Ok(writer)
}

pub fn from_str<T: serde::de::DeserializeOwned>(s: &str) -> Result<T, Error> {
    let mut docs = YamlLoader::load_from_str(s)?;
    if docs.is_empty() {
        return Err(Error::Custom("no documents".to_string()));
    }
    let yaml = docs.remove(0);
    T::deserialize(de::YamlDeserializer::new(&yaml))
}

pub fn to_value<T: serde::Serialize>(value: &T) -> Result<Value, Error> {
    let yaml = value.serialize(ser::YamlSerializer::new())?;
    Ok(Value::from_yaml(&yaml))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        let yaml_val = Yaml::String("test".to_string());
        assert_eq!(yaml_val.as_str().unwrap(), "test");
    }

    #[test]
    fn test_yaml_from_str() {
        assert_eq!(Yaml::from_str("42"), Yaml::Integer(42));
        assert_eq!(Yaml::from_str("true"), Yaml::Boolean(true));
        assert_eq!(Yaml::from_str("false"), Yaml::Boolean(false));
        assert_eq!(Yaml::from_str("null"), Yaml::Null);
        assert_eq!(Yaml::from_str("~"), Yaml::Null);
        assert_eq!(Yaml::from_str("3.14"), Yaml::Real("3.14".to_string()));
        assert_eq!(Yaml::from_str("hello"), Yaml::String("hello".to_string()));
    }

    #[test]
    fn test_yaml_types() {
        let null_yaml = Yaml::Null;
        assert!(null_yaml.is_null());

        let bool_yaml = Yaml::Boolean(true);
        assert_eq!(bool_yaml.as_bool(), Some(true));

        let int_yaml = Yaml::Integer(42);
        assert_eq!(int_yaml.as_i64(), Some(42));

        let string_yaml = Yaml::String("test".to_string());
        assert_eq!(string_yaml.as_str(), Some("test"));
    }

    #[test]
    fn test_linked_hash_map() {
        let mut map = LinkedHashMap::new();
        map.insert("first".to_string(), 1);
        map.insert("second".to_string(), 2);
        map.insert("third".to_string(), 3);

        assert_eq!(map.len(), 3);
        assert_eq!(map.get("first"), Some(&1));
        assert_eq!(map.get("second"), Some(&2));
        assert_eq!(map.get("third"), Some(&3));

        let keys: Vec<_> = map.iter().map(|(k, _)| k.clone()).collect();
        assert_eq!(keys, vec!["first", "second", "third"]);
    }

    #[test]
    fn test_simple_key_value() {
        let s = "key: value";
        let result = YamlLoader::load_from_str(s);
        match result {
            Ok(docs) => {
                assert_eq!(docs.len(), 1);
                let doc = &docs[0];
                assert_eq!(doc["key"].as_str().unwrap(), "value");
            }
            Err(e) => {
                panic!("Parsing failed: {}", e);
            }
        }
    }

    #[test]
    fn test_simple_doc() {
        let s = "hello: world
int: 42
bool: true
nulltest: ~";
        let result = YamlLoader::load_from_str(s);
        match result {
            Ok(docs) => {
                assert_eq!(docs.len(), 1);
                let doc = &docs[0];
                assert_eq!(doc["hello"].as_str().unwrap(), "world");
                assert_eq!(doc["int"].as_i64().unwrap(), 42);
                assert_eq!(doc["bool"].as_bool().unwrap(), true);
                assert!(doc["nulltest"].is_null());
            }
            Err(e) => {
                panic!("Parsing failed: {}", e);
            }
        }
    }

    #[test]
    fn test_flow_seq() {
        let s = "[1, 2, 3]";
        let result = YamlLoader::load_from_str(s);
        match result {
            Ok(docs) => {
                let arr = docs[0].as_vec().unwrap();
                assert_eq!(arr.len(), 3);
                assert_eq!(arr[0].as_i64().unwrap(), 1);
                assert_eq!(arr[1].as_i64().unwrap(), 2);
                assert_eq!(arr[2].as_i64().unwrap(), 3);
            }
            Err(e) => {
                panic!("Flow sequence parsing failed: {}", e);
            }
        }
    }

    #[test]
    fn test_two_line_mapping() {
        let s = "hello: world\nint: 42";
        let result = YamlLoader::load_from_str(s);
        match result {
            Ok(docs) => {
                assert_eq!(docs.len(), 1);
                let doc = &docs[0];
                assert_eq!(doc["hello"].as_str().unwrap(), "world");
                assert_eq!(doc["int"].as_i64().unwrap(), 42);
            }
            Err(e) => {
                panic!("Two-line mapping failed: {}", e);
            }
        }
    }
}
