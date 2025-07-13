use std::collections::BTreeMap;

/// Maintains insertion order plus unique keys, like `linked_hash_map`.
/// Here we inline a minimal version for demonstration.
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct LinkedHashMap<K: PartialEq + Eq, V> {
    map: BTreeMap<usize, (K, V)>,
    order: Vec<usize>,
    next_id: usize,
}

impl<K: PartialEq + Eq, V> LinkedHashMap<K, V> {
    pub fn new() -> Self {
        LinkedHashMap {
            map: BTreeMap::new(),
            order: Vec::new(),
            next_id: 0,
        }
    }
    
    pub fn is_empty(&self) -> bool {
        self.order.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.order.len()
    }
    
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: PartialEq + ?Sized,
    {
        for id in &self.order {
            if let Some((ref k, ref v)) = self.map.get(id) {
                if k.borrow() == key {
                    return Some(v);
                }
            }
        }
        None
    }
    
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // check if key exists
        for id in &self.order {
            if let Some((ref k, ref mut old_v)) = self.map.get_mut(id) {
                if k == &key {
                    let old = std::mem::replace(old_v, value);
                    return Some(old);
                }
            }
        }
        let id = self.next_id;
        self.next_id += 1;
        self.map.insert(id, (key, value));
        self.order.push(id);
        None
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.order
            .iter()
            .filter_map(move |id| {
                self.map.get(id).map(|(k, v)| (k, v))
            })
    }
}

impl<K: PartialEq + Eq, V> Default for LinkedHashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: PartialEq + Eq + Clone, V: Clone> IntoIterator for LinkedHashMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        let mut out = Vec::with_capacity(self.order.len());
        for id in self.order {
            if let Some((k, v)) = self.map.get(&id) {
                out.push((k.clone(), v.clone()));
            }
        }
        out.into_iter()
    }
} 