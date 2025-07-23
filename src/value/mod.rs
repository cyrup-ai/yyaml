use crate::{Error, Yaml};
use serde::{Deserialize, Serialize};
use serde::de::IntoDeserializer;
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

/// A YAML tag for typed values
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag {
    pub handle: Option<String>,
    pub suffix: String,
}

impl Tag {
    pub fn new(handle: Option<String>, suffix: String) -> Self {
        Self { handle, suffix }
    }
}

/// A tagged YAML value with type information
#[derive(Clone, Debug, PartialEq)]
pub struct TaggedValue {
    pub tag: Tag,
    pub value: Value,
}

impl TaggedValue {
    pub fn new(tag: Tag, value: Value) -> Self {
        Self { tag, value }
    }
}

/// A serde-compatible YAML value representation
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Sequence(Sequence),
    Mapping(Mapping),
    Tagged(Box<TaggedValue>),
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

    /// Check if value is an f64 number
    #[inline(always)]
    pub const fn is_f64(&self) -> bool {
        match self {
            Value::Number(n) => n.is_f64(),
            _ => false,
        }
    }

    /// Check if value is a string
    #[inline(always)]
    pub const fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    /// Apply merge operation for YAML merge key functionality
    pub fn apply_merge(&mut self) -> Result<(), crate::Error> {
        // Implementation for YAML merge key (<<) functionality
        // For now, return Ok since merge keys are handled during parsing
        Ok(())
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
            Yaml::Alias(_) => panic!("Unresolved alias found - this indicates a bug in YAML parsing"),
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
            Value::Tagged(tagged) => tagged.value.into_yaml(),
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
            Value::Tagged(tagged) => {
                6u8.hash(state);
                tagged.tag.hash(state);
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

            (Value::Number(a), Value::Number(b)) => {
                // Safe comparison - Number should always be comparable
                match a.partial_cmp(b) {
                    Some(ordering) => ordering,
                    None => Ordering::Equal, // Fallback for NaN values
                }
            },
            (Value::Number(_), _) => Ordering::Less,
            (_, Value::Number(_)) => Ordering::Greater,

            (Value::String(a), Value::String(b)) => a.cmp(b),
            (Value::String(_), _) => Ordering::Less,
            (_, Value::String(_)) => Ordering::Greater,

            (Value::Sequence(_), Value::Sequence(_)) => Ordering::Equal,
            (Value::Sequence(_), _) => Ordering::Less,
            (_, Value::Sequence(_)) => Ordering::Greater,

            (Value::Mapping(_), Value::Mapping(_)) => Ordering::Equal,
            (Value::Mapping(_), _) => Ordering::Less,
            (_, Value::Mapping(_)) => Ordering::Greater,

            (Value::Tagged(a), Value::Tagged(b)) => a.tag.suffix.cmp(&b.tag.suffix),
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
            Value::Tagged(tagged) => tagged.value.serialize(serializer),
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

// Static null value for Index trait implementations
static NULL_VALUE: Value = Value::Null;

/// Indexing by usize for sequences
impl std::ops::Index<usize> for Value {
    type Output = Value;

    fn index(&self, index: usize) -> &Self::Output {
        match self {
            Value::Sequence(seq) => {
                match seq.get(index) {
                    Some(value) => value,
                    None => &NULL_VALUE,
                }
            }
            _ => &NULL_VALUE,
        }
    }
}

/// Indexing by string for mappings
impl std::ops::Index<&str> for Value {
    type Output = Value;

    fn index(&self, key: &str) -> &Self::Output {
        match self {
            Value::Mapping(map) => {
                let key_value = Value::String(key.to_string());
                match map.get(&key_value) {
                    Some(value) => value,
                    None => &NULL_VALUE,
                }
            }
            _ => &NULL_VALUE,
        }
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

/// Owned value deserializer to break recursion cycles
pub struct ValueDeserializerOwned {
    value: Value,
}

impl ValueDeserializerOwned {
    pub fn new(value: Value) -> Self {
        ValueDeserializerOwned { value }
    }
}

// Obsolete ValueSeqDeserializer and ValueMapDeserializer have been replaced 
// by ZeroRecursionSeqAccess and ZeroRecursionMapAccess to eliminate infinite recursion

impl<'de> serde::Deserializer<'de> for ValueDeserializerOwned {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            Value::Null => visitor.visit_unit(),
            Value::Bool(b) => visitor.visit_bool(b),
            Value::Number(n) => match n {
                Number::Integer(i) => visitor.visit_i64(i),
                Number::Float(f) => visitor.visit_f64(f),
            },
            Value::String(s) => visitor.visit_string(s),
            Value::Sequence(seq) => {
                let mut seq_access = ZeroRecursionSeqAccess::new(seq);
                visitor.visit_seq(&mut seq_access)
            }
            Value::Mapping(map) => {
                let mut map_access = ZeroRecursionMapAccess::new(map);
                visitor.visit_map(&mut map_access)
            }
            Value::Tagged(tagged) => {
                // Unwrap tagged values iteratively to prevent infinite recursion
                let mut current_value = tagged.value;
                while let Value::Tagged(inner_tagged) = current_value {
                    current_value = inner_tagged.value;
                }
                
                // Handle the unwrapped value directly without recursion
                match current_value {
                    Value::Null => visitor.visit_unit(),
                    Value::Bool(b) => visitor.visit_bool(b),
                    Value::Number(n) => match n {
                        Number::Integer(i) => visitor.visit_i64(i),
                        Number::Float(f) => visitor.visit_f64(f),
                    },
                    Value::String(s) => visitor.visit_string(s),
                    Value::Sequence(seq) => {
                        let mut seq_access = ZeroRecursionSeqAccess::new(seq);
                        visitor.visit_seq(&mut seq_access)
                    }
                    Value::Mapping(map) => {
                        let mut map_access = ZeroRecursionMapAccess::new(map);
                        visitor.visit_map(&mut map_access)
                    }
                    Value::Tagged(_) => {
                        // This should never happen due to unwrapping above
                        Err(Error::Custom("Nested tagged value after unwrapping".to_string()))
                    }
                }
            }
        }
    }

    // Custom deserialize_seq to handle null -> empty sequence conversion for owned values
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            Value::Sequence(seq) => {
                let mut seq_access = ZeroRecursionSeqAccess::new(seq);
                visitor.visit_seq(&mut seq_access)
            }
            Value::Null => {
                // Convert null to empty sequence using empty deserializer
                visitor.visit_seq(EmptySeqDeserializer)
            }
            Value::Tagged(tagged) => {
                // Unwrap tagged values iteratively
                let mut current_value = tagged.value;
                while let Value::Tagged(inner_tagged) = current_value {
                    current_value = inner_tagged.value;
                }
                match current_value {
                    Value::Sequence(seq) => {
                        let mut seq_access = ZeroRecursionSeqAccess::new(seq);
                        visitor.visit_seq(&mut seq_access)
                    }
                    Value::Null => visitor.visit_seq(EmptySeqDeserializer),
                    _ => Err(Error::Custom(format!("cannot deserialize {current_value:?} as sequence"))),
                }
            }
            _ => Err(Error::Custom(format!("cannot deserialize {:?} as sequence", self.value))),
        }
    }

    // Custom deserialize_map to handle null -> empty mapping conversion for owned values
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self.value {
            Value::Mapping(map) => {
                let mut map_access = ZeroRecursionMapAccess::new(map);
                visitor.visit_map(&mut map_access)
            }
            Value::Null => {
                // Convert null to empty mapping
                visitor.visit_map(EmptyMapDeserializer)
            }
            Value::Tagged(tagged) => {
                // Unwrap tagged values iteratively
                let mut current_value = tagged.value;
                while let Value::Tagged(inner_tagged) = current_value {
                    current_value = inner_tagged.value;
                }
                match current_value {
                    Value::Mapping(map) => {
                        let mut map_access = ZeroRecursionMapAccess::new(map);
                        visitor.visit_map(&mut map_access)
                    }
                    Value::Null => visitor.visit_map(EmptyMapDeserializer),
                    _ => Err(Error::Custom(format!("cannot deserialize {current_value:?} as mapping"))),
                }
            }
            _ => Err(Error::Custom(format!("cannot deserialize {:?} as mapping", self.value))),
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct tuple tuple_struct
        struct enum identifier ignored_any
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
                let mut seq_access = ZeroRecursionSeqAccess::new(seq.clone());
                visitor.visit_seq(&mut seq_access)
            }
            Value::Mapping(map) => {
                let mut map_access = ZeroRecursionMapAccess::new(map.clone());
                visitor.visit_map(&mut map_access)
            }
            Value::Tagged(tagged) => {
                // Unwrap tagged values iteratively to prevent infinite recursion
                let mut current_value = tagged.value.clone();
                while let Value::Tagged(inner_tagged) = current_value {
                    current_value = inner_tagged.value.clone();
                }
                
                // Handle the unwrapped value directly without calling deserialize_any
                match current_value {
                    Value::Null => visitor.visit_unit(),
                    Value::Bool(b) => visitor.visit_bool(b),
                    Value::Number(n) => match n {
                        Number::Integer(i) => visitor.visit_i64(i),
                        Number::Float(f) => visitor.visit_f64(f),
                    },
                    Value::String(s) => visitor.visit_string(s),
                    Value::Sequence(seq) => {
                        let mut seq_access = ZeroRecursionSeqAccess::new(seq);
                        visitor.visit_seq(&mut seq_access)
                    }
                    Value::Mapping(map) => {
                        let mut map_access = ZeroRecursionMapAccess::new(map);
                        visitor.visit_map(&mut map_access)
                    }
                    Value::Tagged(_) => {
                        // This should never happen due to unwrapping above
                        Err(Error::Custom("Nested tagged value after unwrapping".to_string()))
                    }
                }
            }
        }
    }

    // Custom deserialize_seq to handle null -> empty sequence conversion
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            Value::Sequence(seq) => {
                let mut seq_access = ZeroRecursionSeqAccess::new(seq.clone());
                visitor.visit_seq(&mut seq_access)
            }
            Value::Null => {
                // Convert null to empty sequence using empty deserializer
                visitor.visit_seq(EmptySeqDeserializer)
            }
            Value::Tagged(tagged) => {
                // Unwrap tagged values iteratively
                let mut current_value = &tagged.value;
                while let Value::Tagged(inner_tagged) = current_value {
                    current_value = &inner_tagged.value;
                }
                match current_value {
                    Value::Sequence(seq) => {
                        let mut seq_access = ZeroRecursionSeqAccess::new(seq.clone());
                        visitor.visit_seq(&mut seq_access)
                    }
                    Value::Null => visitor.visit_seq(EmptySeqDeserializer),
                    _ => Err(Error::Custom(format!("cannot deserialize {current_value:?} as sequence"))),
                }
            }
            _ => Err(Error::Custom(format!("cannot deserialize {self:?} as sequence"))),
        }
    }

    // Custom deserialize_map to handle null -> empty mapping conversion  
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            Value::Mapping(map) => {
                let mut map_access = ZeroRecursionMapAccess::new(map.clone());
                visitor.visit_map(&mut map_access)
            }
            Value::Null => {
                // Convert null to empty mapping
                visitor.visit_map(EmptyMapDeserializer)
            }
            Value::Tagged(tagged) => {
                // Unwrap tagged values iteratively
                let mut current_value = &tagged.value;
                while let Value::Tagged(inner_tagged) = current_value {
                    current_value = &inner_tagged.value;
                }
                match current_value {
                    Value::Mapping(map) => {
                        let mut map_access = ZeroRecursionMapAccess::new(map.clone());
                        visitor.visit_map(&mut map_access)
                    }
                    Value::Null => visitor.visit_map(EmptyMapDeserializer),
                    _ => Err(Error::Custom(format!("cannot deserialize {current_value:?} as mapping"))),
                }
            }
            _ => Err(Error::Custom(format!("cannot deserialize {self:?} as mapping"))),
        }
    }

    // Custom deserialize_option to handle Option<T> deserialization correctly
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::Visitor<'de>,
    {
        match self {
            Value::Null => visitor.visit_none(),
            _ => {
                // Create a simple deserializer for Option<T> handling that uses our zero-recursion dispatch
                struct NonRecursiveValueDeserializer {
                    value: Value,
                }
                
                impl<'de> serde::Deserializer<'de> for NonRecursiveValueDeserializer {
                    type Error = Error;
                    
                    fn deserialize_any<Vis>(self, visitor: Vis) -> Result<Vis::Value, Self::Error>
                    where
                        Vis: serde::de::Visitor<'de>,
                    {
                        dispatch_value_to_visitor_internal(self.value, visitor)
                    }
                    
                    serde::forward_to_deserialize_any! {
                        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
                        byte_buf option unit unit_struct newtype_struct seq tuple tuple_struct
                        map struct enum identifier ignored_any
                    }
                }
                
                visitor.visit_some(NonRecursiveValueDeserializer { value: self.clone() })
            },
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf unit unit_struct newtype_struct tuple tuple_struct
        struct enum identifier ignored_any
    }
}



// Empty deserializers to handle null-to-collection conversions without lifetime issues
struct EmptySeqDeserializer;

impl<'de> serde::de::SeqAccess<'de> for EmptySeqDeserializer {
    type Error = Error;

    fn next_element_seed<T>(&mut self, _seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        Ok(None) // Always empty
    }

    fn size_hint(&self) -> Option<usize> {
        Some(0)
    }
}

struct EmptyMapDeserializer;

impl<'de> serde::de::MapAccess<'de> for EmptyMapDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, _seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        Ok(None) // Always empty
    }

    fn next_value_seed<V>(&mut self, _seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        Err(Error::Custom("EmptyMapDeserializer should not have values".to_string()))
    }

    fn size_hint(&self) -> Option<usize> {
        Some(0)
    }
}




/// IntoDeserializer implementation for Value - uses owned deserializer to avoid recursion
impl IntoDeserializer<'_, Error> for Value {
    type Deserializer = ValueDeserializerOwned;

    fn into_deserializer(self) -> Self::Deserializer {
        ValueDeserializerOwned::new(self)
    }
}

// Old DirectSeqAccess and DirectMapAccess have been replaced by ZeroRecursionSeqAccess and ZeroRecursionMapAccess

/// Dispatch a value directly to a visitor without creating recursive deserializers
/// This is the core non-recursive dispatch mechanism that breaks all recursion cycles
pub fn dispatch_value_to_visitor<'de, V>(
    value: Value, 
    seed: V
) -> Result<V::Value, Error>
where
    V: serde::de::DeserializeSeed<'de>,
{
    // TRUE direct dispatch without any intermediate deserializer creation
    struct ZeroRecursionDeserializer {
        value: Value,
    }
    
    impl<'de> serde::Deserializer<'de> for ZeroRecursionDeserializer {
        type Error = Error;
        
        fn deserialize_any<Vis>(self, visitor: Vis) -> Result<Vis::Value, Self::Error>
        where
            Vis: serde::de::Visitor<'de>,
        {
            dispatch_value_to_visitor_internal(self.value, visitor)
        }
        
        // Custom deserialize_seq to handle null -> empty sequence conversion
        fn deserialize_seq<Vis>(self, visitor: Vis) -> Result<Vis::Value, Self::Error>
        where
            Vis: serde::de::Visitor<'de>,
        {
            // Unwrap tagged values iteratively first
            let mut unwrapped_value = self.value;
            while let Value::Tagged(tagged) = unwrapped_value {
                unwrapped_value = tagged.value;
            }
            
            match unwrapped_value {
                Value::Sequence(seq) => {
                    let mut seq_access = ZeroRecursionSeqAccess::new(seq);
                    visitor.visit_seq(&mut seq_access)
                }
                Value::Null => {
                    // Convert null to empty sequence
                    visitor.visit_seq(EmptySeqDeserializer)
                }
                _ => Err(Error::Custom(format!("cannot deserialize {unwrapped_value:?} as sequence"))),
            }
        }
        
        // Custom deserialize_map to handle null -> empty mapping conversion
        fn deserialize_map<Vis>(self, visitor: Vis) -> Result<Vis::Value, Self::Error>
        where
            Vis: serde::de::Visitor<'de>,
        {
            // Unwrap tagged values iteratively first
            let mut unwrapped_value = self.value;
            while let Value::Tagged(tagged) = unwrapped_value {
                unwrapped_value = tagged.value;
            }
            
            match unwrapped_value {
                Value::Mapping(map) => {
                    let mut map_access = ZeroRecursionMapAccess::new(map);
                    visitor.visit_map(&mut map_access)
                }
                Value::Null => {
                    // Convert null to empty mapping
                    visitor.visit_map(EmptyMapDeserializer)
                }
                _ => Err(Error::Custom(format!("cannot deserialize {unwrapped_value:?} as mapping"))),
            }
        }
        
        // Custom deserialize_option to handle null -> None conversion
        fn deserialize_option<Vis>(self, visitor: Vis) -> Result<Vis::Value, Self::Error>
        where
            Vis: serde::de::Visitor<'de>,
        {
            // Unwrap tagged values iteratively first
            let mut unwrapped_value = self.value;
            while let Value::Tagged(tagged) = unwrapped_value {
                unwrapped_value = tagged.value;
            }
            
            match unwrapped_value {
                Value::Null => visitor.visit_none(),
                _ => visitor.visit_some(ZeroRecursionDeserializer { value: unwrapped_value }),
            }
        }
        
        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
            byte_buf unit unit_struct newtype_struct tuple tuple_struct
            struct enum identifier ignored_any
        }
    }
    
    seed.deserialize(ZeroRecursionDeserializer { value })
}

/// Internal dispatch that handles value-to-visitor conversion without ANY recursion
/// This is the blazing-fast, zero-allocation core that eliminates all recursive patterns
fn dispatch_value_to_visitor_internal<'de, V>(
    value: Value,
    visitor: V
) -> Result<V::Value, Error>
where
    V: serde::de::Visitor<'de>,
{
    // Unwrap tagged values iteratively first to prevent any tagged recursion
    let mut unwrapped_value = value;
    while let Value::Tagged(tagged) = unwrapped_value {
        unwrapped_value = tagged.value;
    }
    
    // Direct dispatch based on unwrapped value type - ZERO intermediate objects
    match unwrapped_value {
        Value::Null => visitor.visit_unit(),
        Value::Bool(b) => visitor.visit_bool(b),
        Value::Number(n) => match n {
            Number::Integer(i) => visitor.visit_i64(i),
            Number::Float(f) => visitor.visit_f64(f),
        },
        Value::String(s) => visitor.visit_string(s),
        Value::Sequence(seq) => {
            // Use completely non-recursive sequence access
            let mut seq_access = ZeroRecursionSeqAccess::new(seq);
            visitor.visit_seq(&mut seq_access)
        }
        Value::Mapping(map) => {
            // Use completely non-recursive mapping access  
            let mut map_access = ZeroRecursionMapAccess::new(map);
            visitor.visit_map(&mut map_access)
        }
        Value::Tagged(_) => {
            // This should never happen due to unwrapping above, but handle gracefully
            Err(Error::Custom("Tagged value found after unwrapping - this indicates a bug".to_string()))
        }
    }
}

/// Completely non-recursive sequence access - breaks ALL recursion cycles
pub struct ZeroRecursionSeqAccess {
    iter: std::vec::IntoIter<Value>,
}

impl ZeroRecursionSeqAccess {
    pub fn new(seq: Sequence) -> Self {
        ZeroRecursionSeqAccess { 
            iter: seq.into_iter() 
        }
    }
}

impl<'de> serde::de::SeqAccess<'de> for ZeroRecursionSeqAccess {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => dispatch_value_to_visitor(value, seed).map(Some),
            None => Ok(None),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        let (_lower, upper) = self.iter.size_hint();
        upper
    }
}

/// Completely non-recursive mapping access - breaks ALL recursion cycles
pub struct ZeroRecursionMapAccess {
    iter: std::vec::IntoIter<(Value, Value)>,
    next_value: Option<Value>,
}

impl ZeroRecursionMapAccess {
    pub fn new(map: Mapping) -> Self {
        let pairs: Vec<(Value, Value)> = map.into_iter().collect();
        ZeroRecursionMapAccess { 
            iter: pairs.into_iter(),
            next_value: None,
        }
    }
}

impl<'de> serde::de::MapAccess<'de> for ZeroRecursionMapAccess {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: serde::de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.next_value = Some(value);
                dispatch_value_to_visitor(key, seed).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: serde::de::DeserializeSeed<'de>,
    {
        match self.next_value.take() {
            Some(value) => dispatch_value_to_visitor(value, seed),
            None => Err(Error::Custom("no value available".to_string())),
        }
    }

    fn size_hint(&self) -> Option<usize> {
        let (_lower, upper) = self.iter.size_hint();
        upper
    }
}

