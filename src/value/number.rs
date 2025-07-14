use crate::{Error, Yaml};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::fmt::{self, Display};
use std::hash::{Hash, Hasher};
use std::str::FromStr;

/// A numeric value that can be either an integer or a float
#[derive(Clone, Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl Number {
    #[inline(always)]
    pub const fn is_i64(&self) -> bool {
        matches!(self, Number::Integer(_))
    }

    #[inline(always)]
    pub const fn is_f64(&self) -> bool {
        matches!(self, Number::Float(_))
    }

    #[inline(always)]
    pub const fn as_i64(&self) -> Option<i64> {
        match self {
            Number::Integer(i) => Some(*i),
            Number::Float(_) => None,
        }
    }

    #[inline]
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Number::Integer(i) => Some(*i as f64),
            Number::Float(f) => Some(*f),
        }
    }

    #[inline]
    pub(crate) fn parse_real(s: &str) -> Option<Number> {
        match s {
            ".inf" | ".Inf" | ".INF" | "+.inf" | "+.Inf" | "+.INF" => {
                Some(Number::Float(f64::INFINITY))
            }
            "-.inf" | "-.Inf" | "-.INF" => Some(Number::Float(f64::NEG_INFINITY)),
            ".nan" | ".NaN" | ".NAN" => Some(Number::Float(f64::NAN)),
            _ => s.parse::<f64>().ok().map(Number::Float),
        }
    }

    #[inline]
    pub(crate) fn into_yaml(self) -> Yaml {
        match self {
            Number::Integer(i) => Yaml::Integer(i),
            Number::Float(f) => {
                if f.is_nan() {
                    Yaml::Real(".nan".to_string())
                } else if f.is_infinite() {
                    if f.is_sign_positive() {
                        Yaml::Real(".inf".to_string())
                    } else {
                        Yaml::Real("-.inf".to_string())
                    }
                } else {
                    Yaml::Real(f.to_string())
                }
            }
        }
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::Integer(i) => write!(f, "{}", i),
            Number::Float(fl) => {
                if fl.is_nan() {
                    write!(f, ".nan")
                } else if fl.is_infinite() {
                    if fl.is_sign_positive() {
                        write!(f, ".inf")
                    } else {
                        write!(f, "-.inf")
                    }
                } else {
                    write!(f, "{}", fl)
                }
            }
        }
    }
}

impl From<i8> for Number {
    #[inline(always)]
    fn from(i: i8) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<i16> for Number {
    #[inline(always)]
    fn from(i: i16) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<i32> for Number {
    #[inline(always)]
    fn from(i: i32) -> Self {
        Number::Integer(i as i64)
    }
}

impl From<i64> for Number {
    #[inline(always)]
    fn from(i: i64) -> Self {
        Number::Integer(i)
    }
}

impl From<u8> for Number {
    #[inline(always)]
    fn from(u: u8) -> Self {
        Number::Integer(u as i64)
    }
}

impl From<u16> for Number {
    #[inline(always)]
    fn from(u: u16) -> Self {
        Number::Integer(u as i64)
    }
}

impl From<u32> for Number {
    #[inline(always)]
    fn from(u: u32) -> Self {
        Number::Integer(u as i64)
    }
}

impl From<u64> for Number {
    #[inline]
    fn from(u: u64) -> Self {
        if u <= i64::MAX as u64 {
            Number::Integer(u as i64)
        } else {
            Number::Float(u as f64)
        }
    }
}

impl From<f32> for Number {
    #[inline(always)]
    fn from(f: f32) -> Self {
        Number::Float(f as f64)
    }
}

impl From<f64> for Number {
    #[inline(always)]
    fn from(f: f64) -> Self {
        Number::Float(f)
    }
}

impl PartialEq<i64> for Number {
    #[inline]
    fn eq(&self, other: &i64) -> bool {
        match self {
            Number::Integer(i) => i == other,
            Number::Float(f) => *f == *other as f64,
        }
    }
}

impl PartialEq<f64> for Number {
    #[inline]
    fn eq(&self, other: &f64) -> bool {
        match self {
            Number::Float(f) => {
                if f.is_nan() && other.is_nan() {
                    true
                } else {
                    f == other
                }
            }
            Number::Integer(i) => *i as f64 == *other,
        }
    }
}

impl Hash for Number {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Number::Integer(i) => {
                0u8.hash(state);
                i.hash(state);
            }
            Number::Float(f) => {
                1u8.hash(state);
                if f.is_nan() {
                    0u64.hash(state);
                } else {
                    f.to_bits().hash(state);
                }
            }
        }
    }
}

impl PartialOrd for Number {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Number::Integer(a), Number::Integer(b)) => a.partial_cmp(b),
            (Number::Integer(i), Number::Float(f)) => (*i as f64).partial_cmp(f),
            (Number::Float(f), Number::Integer(i)) => f.partial_cmp(&(*i as f64)),
            (Number::Float(a), Number::Float(b)) => a.partial_cmp(b),
        }
    }
}

impl FromStr for Number {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        // Handle special float values
        if let Some(num) = Number::parse_real(s) {
            return Ok(num);
        }

        // Try parsing as integer first
        if let Ok(i) = s.parse::<i64>() {
            return Ok(Number::Integer(i));
        }

        // Try parsing as float
        if let Ok(f) = s.parse::<f64>() {
            return Ok(Number::Float(f));
        }

        Err(Error::Custom("failed to parse YAML number".to_string()))
    }
}

impl Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Number::Integer(i) => serializer.serialize_i64(*i),
            Number::Float(f) => serializer.serialize_f64(*f),
        }
    }
}

impl<'de> Deserialize<'de> for Number {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct NumberVisitor;

        impl<'de> serde::de::Visitor<'de> for NumberVisitor {
            type Value = Number;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number")
            }

            #[inline]
            fn visit_i8<E>(self, value: i8) -> Result<Number, E> {
                Ok(Number::Integer(value as i64))
            }

            #[inline]
            fn visit_i16<E>(self, value: i16) -> Result<Number, E> {
                Ok(Number::Integer(value as i64))
            }

            #[inline]
            fn visit_i32<E>(self, value: i32) -> Result<Number, E> {
                Ok(Number::Integer(value as i64))
            }

            #[inline]
            fn visit_i64<E>(self, value: i64) -> Result<Number, E> {
                Ok(Number::Integer(value))
            }

            #[inline]
            fn visit_i128<E>(self, value: i128) -> Result<Number, E> {
                if value >= i64::MIN as i128 && value <= i64::MAX as i128 {
                    Ok(Number::Integer(value as i64))
                } else {
                    Ok(Number::Float(value as f64))
                }
            }

            #[inline]
            fn visit_u8<E>(self, value: u8) -> Result<Number, E> {
                Ok(Number::Integer(value as i64))
            }

            #[inline]
            fn visit_u16<E>(self, value: u16) -> Result<Number, E> {
                Ok(Number::Integer(value as i64))
            }

            #[inline]
            fn visit_u32<E>(self, value: u32) -> Result<Number, E> {
                Ok(Number::Integer(value as i64))
            }

            #[inline]
            fn visit_u64<E>(self, value: u64) -> Result<Number, E> {
                if value <= i64::MAX as u64 {
                    Ok(Number::Integer(value as i64))
                } else {
                    Ok(Number::Float(value as f64))
                }
            }

            #[inline]
            fn visit_u128<E>(self, value: u128) -> Result<Number, E> {
                if value <= i64::MAX as u128 {
                    Ok(Number::Integer(value as i64))
                } else {
                    Ok(Number::Float(value as f64))
                }
            }

            #[inline]
            fn visit_f32<E>(self, value: f32) -> Result<Number, E> {
                Ok(Number::Float(value as f64))
            }

            #[inline]
            fn visit_f64<E>(self, value: f64) -> Result<Number, E> {
                Ok(Number::Float(value))
            }

            #[inline]
            fn visit_str<E>(self, s: &str) -> Result<Number, E>
            where
                E: serde::de::Error,
            {
                s.parse().map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(NumberVisitor)
    }
}
