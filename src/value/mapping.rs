use crate::{LinkedHashMap, Yaml, value::Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

/// A mapping of YAML values preserving insertion order
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Mapping(BTreeMap<Value, Value>);

impl Mapping {
    #[inline(always)]
    pub const fn new() -> Self {
        Mapping(BTreeMap::new())
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
    pub fn clear(&mut self) {
        self.0.clear()
    }

    #[inline(always)]
    pub fn insert(&mut self, k: Value, v: Value) -> Option<Value> {
        self.0.insert(k, v)
    }

    #[inline(always)]
    pub fn remove(&mut self, k: &Value) -> Option<Value> {
        self.0.remove(k)
    }

    #[inline(always)]
    pub fn get(&self, k: &Value) -> Option<&Value> {
        self.0.get(k)
    }

    #[inline(always)]
    pub fn get_mut(&mut self, k: &Value) -> Option<&mut Value> {
        self.0.get_mut(k)
    }

    #[inline(always)]
    pub fn contains_key(&self, k: &Value) -> bool {
        self.0.contains_key(k)
    }

    #[inline(always)]
    pub fn keys(&self) -> std::collections::btree_map::Keys<Value, Value> {
        self.0.keys()
    }

    #[inline(always)]
    pub fn values(&self) -> std::collections::btree_map::Values<Value, Value> {
        self.0.values()
    }

    #[inline(always)]
    pub fn values_mut(&mut self) -> std::collections::btree_map::ValuesMut<Value, Value> {
        self.0.values_mut()
    }

    #[inline(always)]
    pub fn iter(&self) -> std::collections::btree_map::Iter<Value, Value> {
        self.0.iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> std::collections::btree_map::IterMut<Value, Value> {
        self.0.iter_mut()
    }

    #[inline(always)]
    pub fn entry(&mut self, key: Value) -> std::collections::btree_map::Entry<Value, Value> {
        self.0.entry(key)
    }

    #[inline]
    pub(crate) fn from_yaml_hash(hash: &LinkedHashMap<Yaml, Yaml>) -> Self {
        let map = hash
            .iter()
            .map(|(k, v)| (Value::from_yaml(k), Value::from_yaml(v)))
            .collect();
        Mapping(map)
    }

    #[inline]
    pub(crate) fn into_yaml(self) -> Yaml {
        let hash = self
            .0
            .into_iter()
            .map(|(k, v)| (k.into_yaml(), v.into_yaml()))
            .collect();
        Yaml::Hash(hash)
    }
}

impl From<BTreeMap<Value, Value>> for Mapping {
    #[inline(always)]
    fn from(map: BTreeMap<Value, Value>) -> Self {
        Mapping(map)
    }
}

impl From<Mapping> for BTreeMap<Value, Value> {
    #[inline(always)]
    fn from(mapping: Mapping) -> Self {
        mapping.0
    }
}

impl Deref for Mapping {
    type Target = BTreeMap<Value, Value>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Mapping {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoIterator for Mapping {
    type Item = (Value, Value);
    type IntoIter = std::collections::btree_map::IntoIter<Value, Value>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Mapping {
    type Item = (&'a Value, &'a Value);
    type IntoIter = std::collections::btree_map::Iter<'a, Value, Value>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Mapping {
    type Item = (&'a Value, &'a mut Value);
    type IntoIter = std::collections::btree_map::IterMut<'a, Value, Value>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

impl FromIterator<(Value, Value)> for Mapping {
    #[inline]
    fn from_iter<I: IntoIterator<Item = (Value, Value)>>(iter: I) -> Self {
        Mapping(iter.into_iter().collect())
    }
}

impl Extend<(Value, Value)> for Mapping {
    #[inline]
    fn extend<I: IntoIterator<Item = (Value, Value)>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl Serialize for Mapping {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Mapping {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        BTreeMap::deserialize(deserializer).map(Mapping)
    }
}
