use crate::{Error, Yaml};
use serde::de::{
    self, Deserializer, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess, Visitor,
};

use std::collections::HashMap;

pub struct YamlDeserializer<'de> {
    value: &'de Yaml,
    // Billion laughs protection
    expansion_depth: usize,
    total_expansions: usize,
    // Anchor resolution context
    anchors: Option<&'de HashMap<usize, Yaml>>,
}

// Billion laughs protection limits
const MAX_EXPANSION_DEPTH: usize = 100;
const MAX_TOTAL_EXPANSIONS: usize = 1_000_000;

impl<'de> YamlDeserializer<'de> {
    #[inline]
    pub fn new(value: &'de Yaml) -> Self {
        YamlDeserializer { 
            value,
            expansion_depth: 0,
            total_expansions: 0,
            anchors: None,
        }
    }

    /// Create a new deserializer with explicit expansion tracking
    #[inline]
    fn with_expansion_tracking(value: &'de Yaml, depth: usize, total: usize) -> Self {
        YamlDeserializer {
            value,
            expansion_depth: depth,
            total_expansions: total,
            anchors: None,
        }
    }
}

#[inline]
fn visit_integer<'de, V: Visitor<'de>>(value: i64, visitor: V) -> Result<V::Value, Error> {
    if value < 0 {
        if value >= i8::MIN as i64 {
            visitor.visit_i8(value as i8)
        } else if value >= i16::MIN as i64 {
            visitor.visit_i16(value as i16)
        } else if value >= i32::MIN as i64 {
            visitor.visit_i32(value as i32)
        } else {
            visitor.visit_i64(value)
        }
    } else if value <= u8::MAX as i64 {
        visitor.visit_u8(value as u8)
    } else if value <= u16::MAX as i64 {
        visitor.visit_u16(value as u16)
    } else if value <= u32::MAX as i64 {
        visitor.visit_u32(value as u32)
    } else {
        visitor.visit_u64(value as u64)
    }
}

impl<'de> de::Deserializer<'de> for YamlDeserializer<'de> {
    type Error = Error;

    #[inline]
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Null => visitor.visit_unit(),
            Yaml::Boolean(b) => visitor.visit_bool(*b),
            Yaml::Integer(i) => visit_integer(*i, visitor),
            Yaml::Real(s) => {
                if let Some(f) = crate::yaml::parse_f64(s) {
                    visitor.visit_f64(f)
                } else {
                    visitor.visit_str(s)
                }
            }
            Yaml::String(s) => visitor.visit_str(s),
            Yaml::Array(_) => self.deserialize_seq(visitor),
            Yaml::Hash(_) => self.deserialize_map(visitor),
            Yaml::Tagged(_tag, value) => {
                // Ignore tag and deserialize the underlying value
                YamlDeserializer::with_expansion_tracking(value, self.expansion_depth, self.total_expansions).deserialize_any(visitor)
            }
            Yaml::Alias(_alias_id) => {
                // Aliases should already be resolved by the parser
                // If we encounter an alias here, it means the parser failed to resolve it
                Err(Error::Custom("Unresolved alias encountered in deserializer".to_string()))
            }
            Yaml::BadValue => Err(Error::Custom("bad value encountered".into())),
        }
    }

    #[inline]
    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Boolean(b) => visitor.visit_bool(*b),
            _ => Err(Error::Custom("expected boolean".into())),
        }
    }

    #[inline]
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => {
                if *i >= i8::MIN as i64 && *i <= i8::MAX as i64 {
                    visitor.visit_i8(*i as i8)
                } else {
                    Err(Error::Custom("integer out of i8 range".into()))
                }
            }
            _ => Err(Error::Custom("expected integer".into())),
        }
    }

    #[inline]
    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => {
                if *i >= i16::MIN as i64 && *i <= i16::MAX as i64 {
                    visitor.visit_i16(*i as i16)
                } else {
                    Err(Error::Custom("integer out of i16 range".into()))
                }
            }
            _ => Err(Error::Custom("expected integer".into())),
        }
    }

    #[inline]
    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => {
                if *i >= i32::MIN as i64 && *i <= i32::MAX as i64 {
                    visitor.visit_i32(*i as i32)
                } else {
                    Err(Error::Custom("integer out of i32 range".into()))
                }
            }
            _ => Err(Error::Custom("expected integer".into())),
        }
    }

    #[inline]
    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => visitor.visit_i64(*i),
            _ => Err(Error::Custom("expected integer".into())),
        }
    }

    #[inline]
    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => {
                if *i >= 0 && *i <= u8::MAX as i64 {
                    visitor.visit_u8(*i as u8)
                } else {
                    Err(Error::Custom("integer out of u8 range".into()))
                }
            }
            _ => Err(Error::Custom("expected integer".into())),
        }
    }

    #[inline]
    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => {
                if *i >= 0 && *i <= u16::MAX as i64 {
                    visitor.visit_u16(*i as u16)
                } else {
                    Err(Error::Custom("integer out of u16 range".into()))
                }
            }
            _ => Err(Error::Custom("expected integer".into())),
        }
    }

    #[inline]
    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => {
                if *i >= 0 && *i <= u32::MAX as i64 {
                    visitor.visit_u32(*i as u32)
                } else {
                    Err(Error::Custom("integer out of u32 range".into()))
                }
            }
            _ => Err(Error::Custom("expected integer".into())),
        }
    }

    #[inline]
    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => {
                if *i >= 0 {
                    visitor.visit_u64(*i as u64)
                } else {
                    Err(Error::Custom("negative integer for u64".into()))
                }
            }
            Yaml::Real(s) => s
                .parse::<u64>()
                .map(|v| visitor.visit_u64(v))
                .unwrap_or_else(|_| Err(Error::Custom("invalid u64".into()))),
            _ => Err(Error::Custom("expected integer".into())),
        }
    }

    #[inline]
    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Real(s) => {
                let f = s
                    .parse::<f32>()
                    .map_err(|_| Error::Custom("invalid f32".into()))?;
                visitor.visit_f32(f)
            }
            Yaml::Integer(i) => visitor.visit_f32(*i as f32),
            _ => Err(Error::Custom("expected number".into())),
        }
    }

    #[inline]
    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Real(s) => {
                let f = s
                    .parse::<f64>()
                    .map_err(|_| Error::Custom("invalid f64".into()))?;
                visitor.visit_f64(f)
            }
            Yaml::Integer(i) => visitor.visit_f64(*i as f64),
            _ => Err(Error::Custom("expected number".into())),
        }
    }

    #[inline]
    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::String(s) => {
                let mut chars = s.chars();
                if let Some(ch) = chars.next()
                    && chars.next().is_none() {
                        return visitor.visit_char(ch);
                    }
                Err(Error::Custom("string is not a single character".into()))
            }
            _ => Err(Error::Custom("expected string".into())),
        }
    }

    #[inline]
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::String(s) => visitor.visit_str(s),
            Yaml::Integer(i) => visitor.visit_str(&i.to_string()),
            Yaml::Real(s) => visitor.visit_str(s),
            Yaml::Boolean(b) => visitor.visit_str(if *b { "true" } else { "false" }),
            Yaml::Null => visitor.visit_str(""),
            _ => Err(Error::Custom("cannot convert to string".into())),
        }
    }

    #[inline]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::String(s) => visitor.visit_bytes(s.as_bytes()),
            _ => Err(Error::Custom("expected string for bytes".into())),
        }
    }

    #[inline]
    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    #[inline]
    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    #[inline]
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Null => visitor.visit_unit(),
            _ => Err(Error::Custom("expected null for unit".into())),
        }
    }

    #[inline]
    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    #[inline]
    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    #[inline]
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Array(seq) => visitor.visit_seq(SeqDeserializer::with_expansion_tracking(
                seq.iter(), 
                self.expansion_depth, 
                self.total_expansions
            )),
            Yaml::Null => visitor.visit_seq(SeqDeserializer::with_expansion_tracking(
                [].iter(), 
                self.expansion_depth, 
                self.total_expansions
            )),
            _ => Err(Error::Custom("expected sequence".into())),
        }
    }

    #[inline]
    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Hash(map) => visitor.visit_map(MapDeserializer::with_expansion_tracking(
                map.iter(), 
                self.expansion_depth, 
                self.total_expansions
            )),
            Yaml::Null => {
                let empty: &[(&Yaml, &Yaml)] = &[];
                visitor.visit_map(MapDeserializer::with_expansion_tracking(
                    empty.iter().copied(), 
                    self.expansion_depth, 
                    self.total_expansions
                ))
            }
            _ => Err(Error::Custom("expected mapping".into())),
        }
    }

    #[inline]
    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    #[inline]
    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::String(s) => visitor.visit_enum(StringEnumDeserializer::new(s)),
            Yaml::Hash(map) => {
                if map.len() == 1 {
                    let (key, value) = map
                        .iter()
                        .next()
                        .ok_or_else(|| Error::Custom("empty enum map".into()))?;
                    visitor.visit_enum(MapEnumDeserializer::new(key, value))
                } else {
                    Err(Error::Custom("enum map must have exactly one entry".into()))
                }
            }
            _ => Err(Error::Custom(
                "expected string or single-entry map for enum".into(),
            )),
        }
    }

    #[inline]
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    #[inline]
    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    #[inline]
    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => visitor.visit_i128(*i as i128),
            Yaml::Real(s) => s
                .parse::<i128>()
                .map(|v| visitor.visit_i128(v))
                .unwrap_or_else(|_| Err(Error::Custom("invalid i128".into()))),
            _ => Err(Error::Custom("expected integer".into())),
        }
    }

    #[inline]
    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Yaml::Integer(i) => {
                if *i >= 0 {
                    visitor.visit_u128(*i as u128)
                } else {
                    Err(Error::Custom("negative integer for u128".into()))
                }
            }
            Yaml::Real(s) => s
                .parse::<u128>()
                .map(|v| visitor.visit_u128(v))
                .unwrap_or_else(|_| Err(Error::Custom("invalid u128".into()))),
            _ => Err(Error::Custom("expected integer".into())),
        }
    }
}

pub struct SeqDeserializer<'de, I> {
    iter: I,
    expansion_depth: usize,
    total_expansions: usize,
    _phantom: std::marker::PhantomData<&'de ()>,
}

impl<'de, I> SeqDeserializer<'de, I> {
    #[inline]
    fn new(iter: I) -> Self {
        SeqDeserializer {
            iter,
            expansion_depth: 0,
            total_expansions: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    fn with_expansion_tracking(iter: I, depth: usize, total: usize) -> Self {
        SeqDeserializer {
            iter,
            expansion_depth: depth,
            total_expansions: total,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, I> SeqAccess<'de> for SeqDeserializer<'de, I>
where
    I: Iterator<Item = &'de Yaml>,
{
    type Error = Error;

    #[inline]
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => {
                let deserializer = YamlDeserializer::with_expansion_tracking(
                    value, 
                    self.expansion_depth + 1, 
                    self.total_expansions
                );
                seed.deserialize(deserializer).map(Some)
            }
            None => Ok(None),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.iter.size_hint().1
    }
}

pub struct MapDeserializer<'de, I> {
    iter: I,
    next_value: Option<&'de Yaml>,
    expansion_depth: usize,
    total_expansions: usize,
    _phantom: std::marker::PhantomData<&'de ()>,
}

impl<'de, I> MapDeserializer<'de, I> {
    #[inline]
    fn new(iter: I) -> Self {
        MapDeserializer {
            iter,
            next_value: None,
            expansion_depth: 0,
            total_expansions: 0,
            _phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    fn with_expansion_tracking(iter: I, depth: usize, total: usize) -> Self {
        MapDeserializer {
            iter,
            next_value: None,
            expansion_depth: depth,
            total_expansions: total,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'de, I> MapAccess<'de> for MapDeserializer<'de, I>
where
    I: Iterator<Item = (&'de Yaml, &'de Yaml)>,
{
    type Error = Error;

    #[inline]
    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: de::DeserializeSeed<'de>,
    {
        // Blazing-fast billion laughs protection
        self.total_expansions += 1;
        if self.total_expansions > MAX_TOTAL_EXPANSIONS {
            return Err(Error::repetition_limit_exceeded());
        }

        match self.iter.next() {
            Some((key, value)) => {
                self.next_value = Some(value);
                let deserializer = YamlDeserializer::with_expansion_tracking(
                    key, 
                    self.expansion_depth + 1, 
                    self.total_expansions
                );
                seed.deserialize(deserializer).map(Some)
            }
            None => Ok(None),
        }
    }

    #[inline]
    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        // Blazing-fast billion laughs protection
        self.total_expansions += 1;
        if self.total_expansions > MAX_TOTAL_EXPANSIONS {
            return Err(Error::repetition_limit_exceeded());
        }

        match self.next_value.take() {
            Some(value) => {
                let deserializer = YamlDeserializer::with_expansion_tracking(
                    value, 
                    self.expansion_depth + 1, 
                    self.total_expansions
                );
                seed.deserialize(deserializer)
            }
            None => Err(Error::Custom("no value available".into())),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.iter.size_hint().1
    }
}

pub struct StringEnumDeserializer<'de> {
    value: &'de str,
}

impl<'de> StringEnumDeserializer<'de> {
    #[inline]
    fn new(value: &'de str) -> Self {
        StringEnumDeserializer { value }
    }
}

impl<'de> EnumAccess<'de> for StringEnumDeserializer<'de> {
    type Error = Error;
    type Variant = UnitVariantDeserializer;

    #[inline]
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        use serde::de::value::StrDeserializer;
        let deserializer: StrDeserializer<Error> = self.value.into_deserializer();
        let variant = seed.deserialize(deserializer)?;
        Ok((variant, UnitVariantDeserializer))
    }
}

pub struct MapEnumDeserializer<'de> {
    key: &'de Yaml,
    value: &'de Yaml,
}

impl<'de> MapEnumDeserializer<'de> {
    #[inline]
    fn new(key: &'de Yaml, value: &'de Yaml) -> Self {
        MapEnumDeserializer { key, value }
    }
}

impl<'de> EnumAccess<'de> for MapEnumDeserializer<'de> {
    type Error = Error;
    type Variant = YamlDeserializer<'de>;

    #[inline]
    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: de::DeserializeSeed<'de>,
    {
        let variant = seed.deserialize(YamlDeserializer::new(self.key))?;
        Ok((variant, YamlDeserializer::new(self.value)))
    }
}

pub struct UnitVariantDeserializer;

impl<'de> VariantAccess<'de> for UnitVariantDeserializer {
    type Error = Error;

    #[inline]
    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    #[inline]
    fn newtype_variant_seed<T>(self, _seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        Err(Error::Custom("unit variant cannot be newtype".into()))
    }

    #[inline]
    fn tuple_variant<V>(self, _len: usize, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("unit variant cannot be tuple".into()))
    }

    #[inline]
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(Error::Custom("unit variant cannot be struct".into()))
    }
}

impl<'de> VariantAccess<'de> for YamlDeserializer<'de> {
    type Error = Error;

    #[inline]
    fn unit_variant(self) -> Result<(), Self::Error> {
        match self.value {
            Yaml::Null => Ok(()),
            _ => Err(Error::Custom("expected null for unit variant".into())),
        }
    }

    #[inline]
    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(self)
    }

    #[inline]
    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    #[inline]
    fn struct_variant<V>(
        self,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }
}
