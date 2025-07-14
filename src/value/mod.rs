use crate::{Error, Yaml};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash::{Hash, Hasher};

mod deserializer;
mod mapping;
mod number;
mod sequence;

pub use deserializer::Deserializer;
pub use mapping::Mapping;
pub use number::Number;
pub use sequence::Sequence;

/// A serde-compatible YAML value representation
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Sequence(Sequence),
    Mapping(Mapping),
}

impl Value {
    #[inline(always)]
    pub const fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    #[inline(always)]
    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn as_number(&self) -> Option<&Number> {
        match self {
            Value::Number(n) => Some(n),
            _ => None,
        }
    }

    #[inline(always)]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn as_sequence(&self) -> Option<&Sequence> {
        match self {
            Value::Sequence(seq) => Some(seq),
            _ => None,
        }
    }

    #[inline(always)]
    pub const fn as_mapping(&self) -> Option<&Mapping> {
        match self {
            Value::Mapping(map) => Some(map),
            _ => None,
        }
    }

    #[inline]
    pub fn from_yaml(yaml: &Yaml) -> Self {
        match yaml {
            Yaml::Null => Value::Null,
            Yaml::Boolean(b) => Value::Bool(*b),
            Yaml::Integer(i) => Value::Number(Number::from(*i)),
            Yaml::Real(s) => Number::parse_real(s)
                .map(Value::Number)
                .unwrap_or_else(|| Value::String(s.clone())),
            Yaml::String(s) => Value::String(s.clone()),
            Yaml::Array(arr) => Value::Sequence(Sequence::from_yaml_array(arr)),
            Yaml::Hash(map) => Value::Mapping(Mapping::from_yaml_hash(map)),
            Yaml::Alias(_) => Value::String("~alias~".to_string()),
            Yaml::BadValue => Value::Null,
        }
    }

    #[inline]
    pub fn into_yaml(self) -> Yaml {
        match self {
            Value::Null => Yaml::Null,
            Value::Bool(b) => Yaml::Boolean(b),
            Value::Number(n) => n.into_yaml(),
            Value::String(s) => Yaml::String(s),
            Value::Sequence(seq) => seq.into_yaml(),
            Value::Mapping(map) => map.into_yaml(),
        }
    }
}

impl Default for Value {
    #[inline(always)]
    fn default() -> Self {
        Value::Null
    }
}

impl From<bool> for Value {
    #[inline(always)]
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl From<i64> for Value {
    #[inline(always)]
    fn from(i: i64) -> Self {
        Value::Number(Number::from(i))
    }
}

impl From<f64> for Value {
    #[inline(always)]
    fn from(f: f64) -> Self {
        Value::Number(Number::from(f))
    }
}

impl From<String> for Value {
    #[inline(always)]
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<&str> for Value {
    #[inline(always)]
    fn from(s: &str) -> Self {
        Value::String(s.to_owned())
    }
}

impl From<Sequence> for Value {
    #[inline(always)]
    fn from(seq: Sequence) -> Self {
        Value::Sequence(seq)
    }
}

impl From<Mapping> for Value {
    #[inline(always)]
    fn from(map: Mapping) -> Self {
        Value::Mapping(map)
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Null => 0u8.hash(state),
            Value::Bool(b) => {
                1u8.hash(state);
                b.hash(state);
            }
            Value::Number(n) => {
                2u8.hash(state);
                n.hash(state);
            }
            Value::String(s) => {
                3u8.hash(state);
                s.hash(state);
            }
            Value::Sequence(_) => {
                4u8.hash(state);
            }
            Value::Mapping(_) => {
                5u8.hash(state);
            }
        }
    }
}

impl Eq for Value {}

impl PartialOrd for Value {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;

        match (self, other) {
            (Value::Null, Value::Null) => Ordering::Equal,
            (Value::Null, _) => Ordering::Less,
            (_, Value::Null) => Ordering::Greater,

            (Value::Bool(a), Value::Bool(b)) => a.cmp(b),
            (Value::Bool(_), _) => Ordering::Less,
            (_, Value::Bool(_)) => Ordering::Greater,

            (Value::Number(a), Value::Number(b)) => a.partial_cmp(b).unwrap_or(Ordering::Equal),
            (Value::Number(_), _) => Ordering::Less,
            (_, Value::Number(_)) => Ordering::Greater,

            (Value::String(a), Value::String(b)) => a.cmp(b),
            (Value::String(_), _) => Ordering::Less,
            (_, Value::String(_)) => Ordering::Greater,

            (Value::Sequence(_), Value::Sequence(_)) => Ordering::Equal,
            (Value::Sequence(_), _) => Ordering::Less,
            (_, Value::Sequence(_)) => Ordering::Greater,

            (Value::Mapping(_), Value::Mapping(_)) => Ordering::Equal,
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Null => serializer.serialize_unit(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => n.serialize(serializer),
            Value::String(s) => serializer.serialize_str(s),
            Value::Sequence(seq) => seq.serialize(serializer),
            Value::Mapping(map) => map.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ValueVisitor;

        impl<'de> serde::de::Visitor<'de> for ValueVisitor {
            type Value = Value;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("any valid YAML value")
            }

            #[inline]
            fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
                Ok(Value::Bool(value))
            }

            #[inline]
            fn visit_i8<E>(self, value: i8) -> Result<Value, E> {
                Ok(Value::Number(Number::from(value as i64)))
            }

            #[inline]
            fn visit_i16<E>(self, value: i16) -> Result<Value, E> {
                Ok(Value::Number(Number::from(value as i64)))
            }

            #[inline]
            fn visit_i32<E>(self, value: i32) -> Result<Value, E> {
                Ok(Value::Number(Number::from(value as i64)))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
                Ok(Value::Number(Number::from(value)))
            }

            #[inline]
            fn visit_i128<E>(self, value: i128) -> Result<Value, E> {
                if value >= i64::MIN as i128 && value <= i64::MAX as i128 {
                    Ok(Value::Number(Number::from(value as i64)))
                } else {
                    Ok(Value::Number(Number::from(value as f64)))
                }
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> Result<Value, E> {
                Ok(Value::Number(Number::from(value as i64)))
            }

            #[inline]
            fn visit_u16<E>(self, value: u16) -> Result<Value, E> {
                Ok(Value::Number(Number::from(value as i64)))
            }

            #[inline]
            fn visit_u32<E>(self, value: u32) -> Result<Value, E> {
                Ok(Value::Number(Number::from(value as i64)))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
                if value <= i64::MAX as u64 {
                    Ok(Value::Number(Number::from(value as i64)))
                } else {
                    Ok(Value::Number(Number::from(value as f64)))
                }
            }

            #[inline]
            fn visit_u128<E>(self, value: u128) -> Result<Value, E> {
                if value <= i64::MAX as u128 {
                    Ok(Value::Number(Number::from(value as i64)))
                } else {
                    Ok(Value::Number(Number::from(value as f64)))
                }
            }

            #[inline]
            fn visit_f32<E>(self, value: f32) -> Result<Value, E> {
                Ok(Value::Number(Number::from(value as f64)))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Value, E> {
                Ok(Value::Number(Number::from(value)))
            }

            #[inline]
            fn visit_str<E>(self, value: &str) -> Result<Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Value::String(value.to_owned()))
            }

            #[inline]
            fn visit_string<E>(self, value: String) -> Result<Value, E> {
                Ok(Value::String(value))
            }

            #[inline]
            fn visit_unit<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_none<E>(self) -> Result<Value, E> {
                Ok(Value::Null)
            }

            #[inline]
            fn visit_some<D>(self, deserializer: D) -> Result<Value, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Value::deserialize(deserializer)
            }

            #[inline]
            fn visit_seq<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let mut vec = if let Some(hint) = visitor.size_hint() {
                    Vec::with_capacity(hint)
                } else {
                    Vec::new()
                };

                while let Some(elem) = visitor.next_element()? {
                    vec.push(elem);
                }

                Ok(Value::Sequence(Sequence::from(vec)))
            }

            #[inline]
            fn visit_map<V>(self, mut visitor: V) -> Result<Value, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut map = Mapping::new();

                while let Some((key, value)) = visitor.next_entry()? {
                    map.insert(key, value);
                }

                Ok(Value::Mapping(map))
            }
        }

        deserializer.deserialize_any(ValueVisitor)
    }
}

/// Deserialize from a Value
#[inline]
pub fn from_value<T>(value: Value) -> Result<T, Error>
where
    T: for<'de> Deserialize<'de>,
{
    let yaml = value.into_yaml();
    T::deserialize(crate::de::YamlDeserializer::new(&yaml))
}

impl<'de> Deserialize<'de> for Yaml {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Ok(value.into_yaml())
    }
}

impl Serialize for Yaml {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = Value::from_yaml(self);
        value.serialize(serializer)
    }
}

/// Deserializer implementation for Value references
#[allow(dead_code)] // May be used for future deserialization extensions
struct ValueDeserializer<'a> {
    value: &'a Value,
}

impl<'a> ValueDeserializer<'a> {
    #[allow(dead_code)] // May be used for future deserialization extensions
    fn new(value: &'a Value) -> Self {
        ValueDeserializer { value }
    }
}

impl<'de> serde::Deserializer<'de> for &'de Value {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_unit(),
            Value::Bool(b) => visitor.visit_bool(*b),
            Value::Number(n) => match n {
                Number::Integer(i) => visitor.visit_i64(*i),
                Number::Float(f) => visitor.visit_f64(*f),
            },
            Value::String(s) => visitor.visit_str(s),
            Value::Sequence(seq) => {
                let deserializer = SeqDeserializer { iter: seq.iter() };
                visitor.visit_seq(deserializer)
            }
            Value::Mapping(map) => {
                let deserializer = MapDeserializer {
                    iter: map.iter(),
                    next_value: None,
                };
                visitor.visit_map(deserializer)
            }
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
        map struct enum identifier ignored_any
    }
}

struct SeqDeserializer<'a> {
    iter: std::slice::Iter<'a, Value>,
}

impl<'de> serde::de::SeqAccess<'de> for SeqDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(value).map(Some),
            None => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}

struct MapDeserializer<'a> {
    iter: std::collections::btree_map::Iter<'a, Value, Value>,
    next_value: Option<&'a Value>,
}

impl<'de> serde::de::MapAccess<'de> for MapDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.next_value = Some(value);
                seed.deserialize(key).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.next_value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(Error::Custom("no value available".to_string())),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        match self.iter.size_hint() {
            (lower, Some(upper)) if lower == upper => Some(upper),
            _ => None,
        }
    }
}
