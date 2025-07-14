use crate::{Yaml, value::Value};
use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

/// A sequence of YAML values
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Sequence(Vec<Value>);

impl Sequence {
    #[inline(always)]
    pub const fn new() -> Self {
        Sequence(Vec::new())
    }

    #[inline(always)]
    pub fn with_capacity(cap: usize) -> Self {
        Sequence(Vec::with_capacity(cap))
    }

    /// Create sequence from a vector of values
    #[inline(always)]
    pub fn from_vec(vec: Vec<Value>) -> Self {
        Sequence(vec)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    #[inline(always)]
    pub fn clear(&mut self) {
        self.0.clear()
    }

    #[inline(always)]
    pub fn push(&mut self, value: Value) {
        self.0.push(value);
    }

    #[inline(always)]
    pub fn pop(&mut self) -> Option<Value> {
        self.0.pop()
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&Value> {
        self.0.get(index)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut Value> {
        self.0.get_mut(index)
    }

    #[inline(always)]
    pub fn iter(&self) -> std::slice::Iter<'_, Value> {
        self.0.iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Value> {
        self.0.iter_mut()
    }

    #[inline(always)]
    pub fn insert(&mut self, index: usize, value: Value) {
        self.0.insert(index, value);
    }

    #[inline(always)]
    pub fn remove(&mut self, index: usize) -> Value {
        self.0.remove(index)
    }

    #[inline]
    pub(crate) fn from_yaml_array(arr: &[Yaml]) -> Self {
        let vec = arr.iter().map(Value::from_yaml).collect();
        Sequence(vec)
    }

    #[inline]
    pub(crate) fn into_yaml(self) -> Yaml {
        let arr = self.0.into_iter().map(Value::into_yaml).collect();
        Yaml::Array(arr)
    }
}

impl From<Vec<Value>> for Sequence {
    #[inline(always)]
    fn from(vec: Vec<Value>) -> Self {
        Sequence(vec)
    }
}

impl From<Sequence> for Vec<Value> {
    #[inline(always)]
    fn from(seq: Sequence) -> Self {
        seq.0
    }
}

impl Deref for Sequence {
    type Target = Vec<Value>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Sequence {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Sequence {
    type Item = Value;
    type IntoIter = std::vec::IntoIter<Value>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Sequence {
    type Item = &'a Value;
    type IntoIter = std::slice::Iter<'a, Value>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Sequence {
    type Item = &'a mut Value;
    type IntoIter = std::slice::IterMut<'a, Value>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl FromIterator<Value> for Sequence {
    #[inline]
    fn from_iter<I: IntoIterator<Item = Value>>(iter: I) -> Self {
        Sequence(iter.into_iter().collect())
    }
}

impl Extend<Value> for Sequence {
    #[inline]
    fn extend<I: IntoIterator<Item = Value>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl Serialize for Sequence {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Sequence {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Vec::deserialize(deserializer).map(Sequence)
    }
}
