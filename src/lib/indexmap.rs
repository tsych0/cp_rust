use std::collections::HashMap;

pub enum Entry<'a, K, V> {
    Occupied(OccupiedEntry<'a, K, V>),
    Vacant(VacantEntry<'a, K, V>),
}

pub struct OccupiedEntry<'a, K, V> {
    pub map: &'a mut IndexMap<K, V>,
    pub key: K,
    pub index: usize,
}

pub struct VacantEntry<'a, K, V> {
    pub map: &'a mut IndexMap<K, V>,
    pub key: K,
}

pub struct IndexMap<K, V> {
    pub map: HashMap<K, usize>,
    pub items: Vec<(K, V)>,
}

impl<K: Clone + Eq + std::hash::Hash, V> IndexMap<K, V> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn contains_key(&self, key: &K) -> bool {
        self.map.contains_key(key)
    }

    pub fn get_entry(&self, index: usize) -> Option<(&K, &V)> {
        self.items.get(index).map(|(k, v)| (k, v))
    }

    pub fn get_entry_mut(&mut self, index: usize) -> Option<(&K, &mut V)> {
        self.items.get_mut(index).map(|(k, v)| (&*k, v))
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(&index) = self.map.get(&key) {
            let old_value = std::mem::replace(&mut self.items[index].1, value);
            Some(old_value)
        } else {
            let index = self.items.len();
            self.map.insert(key.clone(), index);
            self.items.push((key, value));
            None
        }
    }

    pub fn get_index(&self, index: usize) -> Option<(&K, &V)> {
        self.items.get(index).map(|(k, v)| (k, v))
    }

    pub fn entry(&mut self, key: K) -> Entry<K, V> {
        if let Some(&index) = self.map.get(&key) {
            Entry::Occupied(OccupiedEntry {
                map: self,
                key,
                index,
            })
        } else {
            Entry::Vacant(VacantEntry { map: self, key })
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.items.iter().map(|(k, v)| (k, v))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.items.iter_mut().map(|(k, v)| (&*k, v))
    }

    pub fn keys(&self) -> impl Iterator<Item = &K> {
        self.items.iter().map(|(k, _)| k)
    }

    pub fn values(&self) -> impl Iterator<Item = &V> {
        self.items.iter().map(|(_, v)| v)
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut V> {
        self.items.iter_mut().map(|(_, v)| v)
    }
}

// You'd need to implement Entry, VacantEntry, and OccupiedEntry types

#[derive(Clone)]
pub struct IndexSet<T> {
    map: HashMap<T, usize>, // value -> index mapping
    items: Vec<T>,
}

impl<T: Clone + Eq + std::hash::Hash> IndexSet<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            items: Vec::new(),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        if self.map.contains_key(&value) {
            false
        } else {
            let index = self.items.len();
            self.map.insert(value.clone(), index);
            self.items.push(value);
            true
        }
    }

    pub fn insert_full(&mut self, value: T) -> (usize, bool) {
        if self.map.contains_key(&value) {
            (*self.map.get(&value).unwrap(), false)
        } else {
            let index = self.items.len();
            self.map.insert(value.clone(), index);
            self.items.push(value);
            (index, true)
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        self.map.contains_key(value)
    }

    pub fn get_full(&self, value: &T) -> Option<(usize, &T)> {
        self.map
            .get(value)
            .map(|&index| (index, &self.items[index]))
    }

    pub fn get_index_of(&self, value: &T) -> Option<usize> {
        self.map.get(value).copied()
    }

    // Remove element by swapping with last element (O(1) but doesn't preserve order)
    pub fn swap_remove(&mut self, value: &T) -> bool {
        if let Some(&index) = self.map.get(value) {
            // Get the last index
            let last_index = self.items.len() - 1;

            if index == last_index {
                // If removing the last element, just pop it
                self.items.pop();
                self.map.remove(value);
            } else {
                // Swap with the last element
                self.items.swap(index, last_index);

                // Update the HashMap: the element that was at last_index is now at index
                let swapped_element = &self.items[index];
                self.map.insert(swapped_element.clone(), index);

                // Remove the target element
                self.items.pop();
                self.map.remove(value);
            }
            true
        } else {
            false
        }
    }

    // Remove element by index with swap (O(1) but doesn't preserve order)
    pub fn swap_remove_index(&mut self, index: usize) -> Option<T> {
        if index >= self.items.len() {
            return None;
        }

        let last_index = self.items.len() - 1;

        if index == last_index {
            // If removing the last element, just pop it
            let removed = self.items.pop().unwrap();
            self.map.remove(&removed);
            Some(removed)
        } else {
            // Get the element to remove
            let removed = self.items[index].clone();

            // Swap with the last element
            self.items.swap(index, last_index);

            // Update the HashMap: the element that was at last_index is now at index
            let swapped_element = &self.items[index];
            self.map.insert(swapped_element.clone(), index);

            // Remove the target element
            self.items.pop();
            self.map.remove(&removed);

            Some(removed)
        }
    }

    // Remove and return the last element (preserves order of remaining elements)
    pub fn pop(&mut self) -> Option<T> {
        if let Some(last_element) = self.items.pop() {
            self.map.remove(&last_element);
            Some(last_element)
        } else {
            None
        }
    }

    // Remove and return the first element (O(n) operation)
    pub fn shift(&mut self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }

        let first_element = self.items.remove(0);

        // Rebuild the HashMap with updated indices
        self.map.clear();
        for (new_index, item) in self.items.iter().enumerate() {
            self.map.insert(item.clone(), new_index);
        }

        Some(first_element)
    }

    // Remove element preserving order (O(n) operation)
    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(&index) = self.map.get(value) {
            self.items.remove(index);

            // Rebuild the HashMap with updated indices
            self.map.clear();
            for (new_index, item) in self.items.iter().enumerate() {
                self.map.insert(item.clone(), new_index);
            }

            true
        } else {
            false
        }
    }

    // Keep only elements that satisfy the predicate
    pub fn retain<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&T) -> bool,
    {
        // Filter the items vector, keeping only elements that satisfy the predicate
        self.items.retain(&mut predicate);

        // Rebuild the HashMap with updated indices
        self.map.clear();
        for (new_index, item) in self.items.iter().enumerate() {
            self.map.insert(item.clone(), new_index);
        }
    }

    // Alternative retain with index access
    pub fn retain_with_index<F>(&mut self, mut predicate: F)
    where
        F: FnMut(usize, &T) -> bool,
    {
        let mut write_index = 0;

        // Iterate through items and keep only those that satisfy the predicate
        for read_index in 0..self.items.len() {
            if predicate(read_index, &self.items[read_index]) {
                if write_index != read_index {
                    self.items[write_index] = self.items[read_index].clone();
                }
                write_index += 1;
            }
        }

        // Truncate to the new length
        self.items.truncate(write_index);

        // Rebuild the HashMap with updated indices
        self.map.clear();
        for (new_index, item) in self.items.iter().enumerate() {
            self.map.insert(item.clone(), new_index);
        }
    }

    // Drain elements that satisfy the predicate (opposite of retain)
    pub fn drain_filter<F>(&mut self, mut predicate: F) -> Vec<T>
    where
        F: FnMut(&T) -> bool,
    {
        let mut drained = Vec::new();
        let mut write_index = 0;

        for read_index in 0..self.items.len() {
            if predicate(&self.items[read_index]) {
                // This element should be drained
                drained.push(self.items[read_index].clone());
            } else {
                // This element should be kept
                if write_index != read_index {
                    self.items[write_index] = self.items[read_index].clone();
                }
                write_index += 1;
            }
        }

        // Truncate to the new length
        self.items.truncate(write_index);

        // Rebuild the HashMap with updated indices
        self.map.clear();
        for (new_index, item) in self.items.iter().enumerate() {
            self.map.insert(item.clone(), new_index);
        }

        drained
    }

    // More efficient retain that avoids cloning during HashMap rebuild
    pub fn retain_efficient<F>(&mut self, predicate: F)
    where
        F: FnMut(&T) -> bool,
        T: std::fmt::Debug, // Only for demonstration - remove if not needed
    {
        let original_len = self.items.len();

        // Use Vec's retain method
        self.items.retain(predicate);

        // Only rebuild HashMap if elements were actually removed
        if self.items.len() != original_len {
            self.map.clear();
            self.map.reserve(self.items.len());

            for (new_index, item) in self.items.iter().enumerate() {
                self.map.insert(item.clone(), new_index);
            }
        }
    }
}

impl<T> IndexSet<T> {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn get_index(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }

    pub fn clear(&mut self) {
        self.items.clear();
        self.map.clear();
    }
}

impl<'a, K: Clone + Eq + std::hash::Hash, V> VacantEntry<'a, K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn index(&self) -> usize {
        self.map.items.len() // This will be the index when inserted
    }

    pub fn insert(self, value: V) -> &'a mut V {
        let index = self.map.items.len();
        self.map.map.insert(self.key.clone(), index);
        self.map.items.push((self.key, value));
        &mut self.map.items[index].1
    }
}

impl<'a, K: Clone + Eq + std::hash::Hash, V> OccupiedEntry<'a, K, V> {
    pub fn key(&self) -> &K {
        &self.key
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn get(&self) -> &V {
        &self.map.items[self.index].1
    }

    pub fn get_mut(&mut self) -> &mut V {
        &mut self.map.items[self.index].1
    }

    pub fn insert(&mut self, value: V) -> V {
        std::mem::replace(&mut self.map.items[self.index].1, value)
    }
}

impl<K, V> IntoIterator for IndexMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// For borrowing references
impl<'a, K, V> IntoIterator for &'a IndexMap<K, V> {
    type Item = (&'a K, &'a V);
    type IntoIter = std::iter::Map<std::slice::Iter<'a, (K, V)>, fn(&'a (K, V)) -> (&'a K, &'a V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter().map(|(k, v)| (k, v))
    }
}

// For mutable borrowing references
impl<'a, K, V> IntoIterator for &'a mut IndexMap<K, V> {
    type Item = (&'a K, &'a mut V);
    type IntoIter =
        std::iter::Map<std::slice::IterMut<'a, (K, V)>, fn(&'a mut (K, V)) -> (&'a K, &'a mut V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter_mut().map(|(k, v)| (&*k, v))
    }
}

impl<K: Clone + Eq + std::hash::Hash, V> std::iter::Extend<(K, V)> for IndexMap<K, V> {
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

// Also implement extend for key-value references
impl<'a, K: Clone + Eq + std::hash::Hash, V: Clone> std::iter::Extend<(&'a K, &'a V)>
    for IndexMap<K, V>
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key.clone(), value.clone());
        }
    }
}

// Additional extend methods for convenience
impl<K: Clone + Eq + std::hash::Hash, V> IndexMap<K, V> {
    pub fn extend<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (K, V)>,
    {
        for (key, value) in iterable {
            self.insert(key, value);
        }
    }

    // Extend from another IndexMap
    pub fn extend_from_map(&mut self, other: &IndexMap<K, V>)
    where
        V: Clone,
    {
        for (key, value) in other.iter() {
            self.insert(key.clone(), value.clone());
        }
    }

    // Extend with reservation for better performance
    pub fn extend_reserve<I>(&mut self, iterable: I)
    where
        I: IntoIterator<Item = (K, V)>,
        I::IntoIter: ExactSizeIterator,
    {
        let iter = iterable.into_iter();
        let additional = iter.len();

        // Reserve space is both Vec and HashMap
        self.items.reserve(additional);
        self.map.reserve(additional);

        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

// FromIterator implementation (bonus!)
impl<K: Clone + Eq + std::hash::Hash, V> std::iter::FromIterator<(K, V)> for IndexMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut map = IndexMap::new();
        map.extend(iter);
        map
    }
}

use std::ops::{Index, IndexMut};

impl<K: Eq + std::hash::Hash, V> Index<&K> for IndexMap<K, V> {
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        match self.map.get(key) {
            Some(&index) => &self.items[index].1,
            None => panic!("Key not found is IndexMap"),
        }
    }
}

impl<K: Eq + std::hash::Hash, V> IndexMut<&K> for IndexMap<K, V> {
    fn index_mut(&mut self, key: &K) -> &mut Self::Output {
        match self.map.get(key) {
            Some(&index) => &mut self.items[index].1,
            None => panic!("Key not found is IndexMap"),
        }
    }
}

// Also implement indexing by usize (positional access)
impl<K, V> Index<usize> for IndexMap<K, V> {
    type Output = V;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index].1
    }
}

impl<K, V> IndexMut<usize> for IndexMap<K, V> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index].1
    }
}

impl<T: Clone + Eq + std::hash::Hash> std::iter::FromIterator<T> for IndexSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = IndexSet::new();
        for item in iter {
            set.insert(item);
        }
        set
    }
}

// Also implement for references
impl<'a, T: Clone + Eq + std::hash::Hash> std::iter::FromIterator<&'a T> for IndexSet<T> {
    fn from_iter<I: IntoIterator<Item = &'a T>>(iter: I) -> Self {
        let mut set = IndexSet::new();
        for item in iter {
            set.insert(item.clone());
        }
        set
    }
}

// Additional FromIterator implementations for convenience
impl<T: Clone + Eq + std::hash::Hash> IndexSet<T> {
    // Create from iterator with capacity hint
    pub fn from_iter_with_capacity<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
        I::IntoIter: ExactSizeIterator,
    {
        let iter = iter.into_iter();
        let capacity = iter.len();

        let mut set = IndexSet::with_capacity(capacity);
        for item in iter {
            set.insert(item);
        }
        set
    }
}

// Don't forget to add with_capacity method to IndexSet
impl<T: Clone + Eq + std::hash::Hash> IndexSet<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity(capacity),
            items: Vec::with_capacity(capacity),
        }
    }
}
