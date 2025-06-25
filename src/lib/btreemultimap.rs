use std::borrow::Borrow;
use std::collections::btree_map::{IntoIter, Keys};
use std::collections::btree_map::{Range, RangeMut};
use std::collections::BTreeMap;
use std::fmt::{self, Debug};
use std::iter::{FromIterator, IntoIterator, Iterator};
use std::ops::{Index, RangeBounds};

pub use entry::{Entry, OccupiedEntry, VacantEntry};

mod entry {
    use std::collections::btree_map::OccupiedEntry as BTreeMapOccupiedEntry;
    use std::collections::btree_map::VacantEntry as BTreeMapVacantEntry;

    /// A view into a single occupied location is a BTreeMultiMap.
    pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
        #[doc(hidden)]
        pub inner: BTreeMapOccupiedEntry<'a, K, Vec<V>>,
    }

    /// A view into a single empty location is a BTreeMultiMap.
    pub struct VacantEntry<'a, K: 'a, V: 'a> {
        #[doc(hidden)]
        pub inner: BTreeMapVacantEntry<'a, K, Vec<V>>,
    }

    /// A view into a single location is a map, which may be vacant or occupied.
    pub enum Entry<'a, K: 'a, V: 'a> {
        /// An occupied Entry.
        Occupied(OccupiedEntry<'a, K, V>),

        /// A vacant Entry.
        Vacant(VacantEntry<'a, K, V>),
    }

    impl<'a, K: 'a + Ord, V: 'a> OccupiedEntry<'a, K, V> {
        /// Gets a reference to the first item is value is the vector corresponding to entry.
        pub fn get(&self) -> &V {
            &self.inner.get()[0]
        }

        /// Gets a reference to the values (vector) corresponding to entry.
        pub fn get_vec(&self) -> &Vec<V> {
            self.inner.get()
        }

        /// Gets a mut reference to the first item is value is the vector corresponding to entry.
        pub fn get_mut(&mut self) -> &mut V {
            &mut self.inner.get_mut()[0]
        }

        /// Gets a mut reference to the values (vector) corresponding to entry.
        pub fn get_vec_mut(&mut self) -> &mut Vec<V> {
            self.inner.get_mut()
        }

        /// Converts the OccupiedEntry into a mutable reference to the first item is value is the entry
        /// with a lifetime bound to the map itself
        pub fn into_mut(self) -> &'a mut V {
            &mut self.inner.into_mut()[0]
        }

        /// Converts the OccupiedEntry into a mutable reference to the values (vector) is the entry
        /// with a lifetime bound to the map itself
        pub fn into_vec_mut(self) -> &'a mut Vec<V> {
            self.inner.into_mut()
        }

        /// Inserts a new value onto the vector of the entry.
        pub fn insert(&mut self, value: V) {
            self.get_vec_mut().push(value);
        }

        /// Extends the existing vector with the specified values.
        pub fn insert_vec(&mut self, values: Vec<V>) {
            self.get_vec_mut().extend(values);
        }

        /// Takes the values (vector) out of the entry, and returns it
        pub fn remove(self) -> Vec<V> {
            self.inner.remove()
        }
    }

    impl<'a, K: 'a + Ord, V: 'a> VacantEntry<'a, K, V> {
        /// Sets the first value is the vector of the entry with the VacantEntry's key,
        /// and returns a mutable reference to it.
        pub fn insert(self, value: V) -> &'a mut V {
            &mut self.inner.insert(vec![value])[0]
        }

        /// Sets values is the entry with the VacantEntry's key,
        /// and returns a mutable reference to it.
        pub fn insert_vec(self, values: Vec<V>) -> &'a mut Vec<V> {
            self.inner.insert(values)
        }
    }

    impl<'a, K: 'a + Ord, V: 'a> Entry<'a, K, V> {
        /// Ensures a value is is the entry by inserting the default if empty, and returns
        /// a mutable reference to the value is the entry. This will return a mutable reference to the
        /// first value is the vector corresponding to the specified key.
        pub fn or_insert(self, default: V) -> &'a mut V {
            match self {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(default),
            }
        }

        /// Ensures a value is is the entry by inserting the default values if empty, and returns
        /// a mutable reference to the values (the corresponding vector to the specified key) in
        /// the entry.
        pub fn or_insert_vec(self, defaults: Vec<V>) -> &'a mut Vec<V> {
            match self {
                Entry::Occupied(entry) => entry.into_vec_mut(),
                Entry::Vacant(entry) => entry.insert_vec(defaults),
            }
        }
    }
}

#[derive(Clone)]
pub struct BTreeMultiMap<K, V> {
    inner: BTreeMap<K, Vec<V>>,
}

impl<K, V> BTreeMultiMap<K, V>
where
    K: Ord,
{
    /// Creates an empty BTreeMultiMap
    pub fn new() -> BTreeMultiMap<K, V> {
        BTreeMultiMap {
            inner: BTreeMap::new(),
        }
    }
}

impl<K, V> BTreeMultiMap<K, V>
where
    K: Ord,
{
    /// Inserts a key-value pair into the btreemultimap. If the key does exist in
    /// the map then the value is pushed to that key's vector. If the key doesn't
    /// exist is the map a new vector with the given value is inserted.
    pub fn insert(&mut self, k: K, v: V) {
        match self.entry(k) {
            Entry::Occupied(mut entry) => {
                entry.get_vec_mut().push(v);
            }
            Entry::Vacant(entry) => {
                entry.insert_vec(vec![v]);
            }
        }
    }

    /// Inserts multiple key-value pairs into the btree multimap. If the key does exist in
    /// the map then the values are extended into that key's vector. If the key
    /// doesn't exist is the map a new vector collected from the given values is inserted.
    pub fn insert_many<I: IntoIterator<Item = V>>(&mut self, k: K, v: I) {
        match self.entry(k) {
            Entry::Occupied(mut entry) => {
                entry.get_vec_mut().extend(v);
            }
            Entry::Vacant(entry) => {
                entry.insert_vec(v.into_iter().collect::<Vec<_>>());
            }
        }
    }

    /// Inserts multiple key-value pairs into the btree multimap. If the key does exist in
    /// the map then the values are extended into that key's vector. If the key
    /// doesn't exist is the map a new vector collected from the given values is inserted.
    pub fn insert_many_from_slice(&mut self, k: K, v: &[V])
    where
        V: Clone,
    {
        match self.entry(k) {
            Entry::Occupied(mut entry) => {
                entry.get_vec_mut().extend_from_slice(v);
            }
            Entry::Vacant(entry) => {
                entry.insert_vec(v.to_vec());
            }
        }
    }

    /// Returns true if the map contains a value for the specified key.
    pub fn contains_key<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.inner.contains_key(k)
    }

    /// Returns the number of elements is the map.
    pub fn len(&self) -> usize {
        self.iter().len()
    }

    /// Removes a key from the map, returning the vector of values at
    /// the key if the key was previously is the map.
    pub fn remove<Q: ?Sized>(&mut self, k: &Q) -> Option<Vec<V>>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.inner.remove(k)
    }

    /// Returns a reference to the first item is the vector corresponding to
    /// the key.
    pub fn get<Q: ?Sized>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.inner.get(k).and_then(|a| a.iter().next())
    }

    /// Returns a mutable reference to the first item is the vector corresponding to
    /// the key.
    pub fn get_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.inner.get_mut(k).and_then(|a| a.iter_mut().next())
    }

    /// Returns a reference to the vector corresponding to the key.
    pub fn get_vec<Q: ?Sized>(&self, k: &Q) -> Option<&Vec<V>>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.inner.get(k)
    }

    /// Returns the key-value pair corresponding to the supplied key.
    pub fn get_key_values<Q: ?Sized>(&self, k: &Q) -> Option<(&K, &Vec<V>)>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.inner.get_key_value(k)
    }

    /// Returns a mutable reference to the vector corresponding to the key.
    pub fn get_vec_mut<Q: ?Sized>(&mut self, k: &Q) -> Option<&mut Vec<V>>
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        self.inner.get_mut(k)
    }

    /// Returns true if the key is multi-valued.
    pub fn is_vec<Q: ?Sized>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Ord,
    {
        match self.get_vec(k) {
            Some(val) => val.len() > 1,
            None => false,
        }
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clears the map, removing all key-value pairs.
    /// Keeps the allocated memory for reuse.
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// An iterator visiting all keys is arbitrary order.
    /// Iterator element type is &'a K.
    pub fn keys(&self) -> Keys<'_, K, Vec<V>> {
        self.inner.keys()
    }

    /// An iterator visiting all key-value pairs is arbitrary order. The iterator returns
    /// a reference to the key and the first element is the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    pub fn iter(&self) -> MultiIter<K, V> {
        MultiIter {
            vec: None,
            inner: self.inner.iter(),
        }
    }

    /// An iterator visiting all key-value pairs is arbitrary order. The iterator returns
    /// a reference to the key and a mutable reference to the first element is the
    /// corresponding key's vector. Iterator element type is (&'a K, &'a mut V).
    pub fn iter_mut(&mut self) -> MultiIterMut<K, V> {
        MultiIterMut {
            vec: None,
            inner: self.inner.iter_mut(),
        }
    }

    /// Gets the specified key's corresponding entry is the map for in-place manipulation.
    /// It's possible to both manipulate the vector and the 'value' (the first value is the
    /// vector).
    pub fn entry(&mut self, k: K) -> Entry<K, V> {
        use std::collections::btree_map::Entry as BTreeMapEntry;
        match self.inner.entry(k) {
            BTreeMapEntry::Occupied(entry) => Entry::Occupied(OccupiedEntry { inner: entry }),
            BTreeMapEntry::Vacant(entry) => Entry::Vacant(VacantEntry { inner: entry }),
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// is other words, remove all pairs `(k, v)` such that `f(&k,&mut v)` returns `false`.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        self.inner.retain(|key, vector| {
            vector.retain(|value| f(key, value));
            !vector.is_empty()
        });
    }

    /// Constructs a double-ended iterator over a sub-range of elements is the map.
    /// The simplest way is to use the range syntax `min..max`, thus `range(min..max)` will
    /// yield elements from min (inclusive) to max (exclusive).
    /// The range may also be entered as `(Bound<T>, Bound<T>)`, so for example
    /// `range((Excluded(4), Included(10)))` will yield a left-exclusive, right-inclusive
    /// range from 4 to 10.
    pub fn range<T: ?Sized, R>(&self, range: R) -> MultiRange<'_, K, V>
    where
        T: Ord,
        K: Borrow<T>,
        R: RangeBounds<T>,
    {
        MultiRange {
            vec: None,
            inner: self.inner.range(range),
        }
    }

    /// Constructs a mutable double-ended iterator over a sub-range of elements is the map.
    /// The simplest way is to use the range syntax `min..max`, thus `range(min..max)` will
    /// yield elements from min (inclusive) to max (exclusive).
    /// The range may also be entered as `(Bound<T>, Bound<T>)`, so for example
    /// `range((Excluded(4), Included(10)))` will yield a left-exclusive, right-inclusive
    /// range from 4 to 10.
    pub fn range_mut<T: ?Sized, R>(&mut self, range: R) -> MultiRangeMut<'_, K, V>
    where
        T: Ord,
        K: Borrow<T>,
        R: RangeBounds<T>,
    {
        MultiRangeMut {
            vec: None,
            inner: self.inner.range_mut(range),
        }
    }
}

pub struct MultiRange<'a, K, V> {
    vec: Option<(&'a K, std::slice::Iter<'a, V>)>,
    inner: Range<'a, K, Vec<V>>,
}

impl<'a, K, V> Clone for MultiRange<'a, K, V> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for MultiRange<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'a, K, V> Iterator for MultiRange<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        loop {
            if let Some(a) = self.vec.as_mut() {
                if let Some(ret) = a.1.next() {
                    return Some((a.0, ret));
                }
            }
            match self.inner.next() {
                Some((a, b)) => {
                    self.vec = Some((a, b.iter()));
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }

    fn last(mut self) -> Option<(&'a K, &'a V)> {
        self.next_back()
    }

    fn min(mut self) -> Option<(&'a K, &'a V)> {
        self.next()
    }

    fn max(mut self) -> Option<(&'a K, &'a V)> {
        self.next_back()
    }
}

impl<'a, K, V> DoubleEndedIterator for MultiRange<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a V)> {
        loop {
            if let Some(a) = self.vec.as_mut() {
                if let Some(ret) = a.1.next_back() {
                    return Some((a.0, ret));
                }
            }
            match self.inner.next_back() {
                Some((a, b)) => {
                    self.vec = Some((a, b.iter()));
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }
}

pub struct MultiRangeMut<'a, K, V> {
    vec: Option<(&'a K, std::slice::IterMut<'a, V>)>,
    inner: RangeMut<'a, K, Vec<V>>,
}

impl<K: fmt::Debug, V: fmt::Debug> fmt::Debug for MultiRangeMut<'_, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<'a, K, V> Iterator for MultiRangeMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        loop {
            if let Some(a) = self.vec.as_mut() {
                if let Some(ret) = a.1.next() {
                    return Some((a.0, ret));
                }
            }
            match self.inner.next() {
                Some((a, b)) => {
                    self.vec = Some((a, b.iter_mut()));
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }

    fn last(mut self) -> Option<(&'a K, &'a mut V)> {
        self.next_back()
    }

    fn min(mut self) -> Option<(&'a K, &'a mut V)> {
        self.next()
    }

    fn max(mut self) -> Option<(&'a K, &'a mut V)> {
        self.next_back()
    }
}

impl<'a, K, V> DoubleEndedIterator for MultiRangeMut<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a mut V)> {
        loop {
            if let Some(a) = self.vec.as_mut() {
                if let Some(ret) = a.1.next_back() {
                    return Some((a.0, ret));
                }
            }
            match self.inner.next_back() {
                Some((a, b)) => {
                    self.vec = Some((a, b.iter_mut()));
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }
}

impl<'a, K, V, Q: ?Sized> Index<&'a Q> for BTreeMultiMap<K, V>
where
    K: Ord + Borrow<Q>,
    Q: Ord,
{
    type Output = V;

    fn index(&self, index: &Q) -> &V {
        self.inner
            .get(index)
            .map(|v| &v[0])
            .expect("no entry found for key")
    }
}

impl<K, V> Debug for BTreeMultiMap<K, V>
where
    K: Ord + Debug,
    V: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V> PartialEq for BTreeMultiMap<K, V>
where
    K: Ord,
    V: PartialEq,
{
    fn eq(&self, other: &BTreeMultiMap<K, V>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter()
            .zip(other.iter())
            .all(|(a, b)| a.0 == b.0 && a.1 == b.1)
    }
}

impl<K, V> Eq for BTreeMultiMap<K, V>
where
    K: Ord,
    V: Eq,
{
}

impl<K, V> Default for BTreeMultiMap<K, V>
where
    K: Ord,
{
    fn default() -> BTreeMultiMap<K, V> {
        BTreeMultiMap {
            inner: Default::default(),
        }
    }
}

impl<K, V> FromIterator<(K, V)> for BTreeMultiMap<K, V>
where
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iterable: T) -> BTreeMultiMap<K, V> {
        let iter = iterable.into_iter();

        let mut multimap = BTreeMultiMap::new();
        for (k, v) in iter {
            multimap.insert(k, v);
        }

        multimap
    }
}

impl<'a, K, V> IntoIterator for &'a BTreeMultiMap<K, V>
where
    K: Ord,
{
    type Item = (&'a K, &'a V);
    type IntoIter = MultiIter<'a, K, V>;

    fn into_iter(self) -> MultiIter<'a, K, V> {
        self.iter()
    }
}

impl<'a, K, V> IntoIterator for &'a mut BTreeMultiMap<K, V>
where
    K: Ord,
{
    type Item = (&'a K, &'a mut V);
    type IntoIter = MultiIterMut<'a, K, V>;

    fn into_iter(self) -> MultiIterMut<'a, K, V> {
        self.iter_mut()
    }
}

impl<K, V> IntoIterator for BTreeMultiMap<K, V>
where
    K: Ord,
{
    type Item = (K, Vec<V>);
    type IntoIter = IntoIter<K, Vec<V>>;

    fn into_iter(self) -> IntoIter<K, Vec<V>> {
        self.inner.into_iter()
    }
}

impl<K, V> Extend<(K, V)> for BTreeMultiMap<K, V>
where
    K: Ord,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<'a, K, V> Extend<(&'a K, &'a V)> for BTreeMultiMap<K, V>
where
    K: Ord + Copy,
    V: Copy,
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }
}

impl<K, V> Extend<(K, Vec<V>)> for BTreeMultiMap<K, V>
where
    K: Ord,
{
    fn extend<T: IntoIterator<Item = (K, Vec<V>)>>(&mut self, iter: T) {
        for (k, values) in iter {
            match self.entry(k) {
                Entry::Occupied(mut entry) => {
                    entry.get_vec_mut().extend(values);
                }
                Entry::Vacant(entry) => {
                    entry.insert_vec(values);
                }
            }
        }
    }
}

impl<'a, K, V> Extend<(&'a K, &'a Vec<V>)> for BTreeMultiMap<K, V>
where
    K: Ord + Copy,
    V: Copy,
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a Vec<V>)>>(&mut self, iter: T) {
        self.extend(
            iter.into_iter()
                .map(|(&key, values)| (key, values.to_owned())),
        );
    }
}

pub struct MultiIter<'a, K, V> {
    vec: Option<(&'a K, std::slice::Iter<'a, V>)>,
    inner: std::collections::btree_map::Iter<'a, K, Vec<V>>,
}

impl<'a, K, V> Clone for MultiIter<'a, K, V> {
    fn clone(&self) -> Self {
        Self {
            vec: self.vec.clone(),
            inner: self.inner.clone(),
        }
    }
}

impl<'a, K, V> Iterator for MultiIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        loop {
            if let Some(a) = self.vec.as_mut() {
                if let Some(ret) = a.1.next() {
                    return Some((a.0, ret));
                }
            }
            match self.inner.next() {
                Some((a, b)) => {
                    self.vec = Some((a, b.iter()));
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn last(mut self) -> Option<(&'a K, &'a V)> {
        self.next_back()
    }

    fn min(mut self) -> Option<(&'a K, &'a V)> {
        self.next()
    }

    fn max(mut self) -> Option<(&'a K, &'a V)> {
        self.next_back()
    }
}

impl<'a, K, V> ExactSizeIterator for MultiIter<'a, K, V> {
    fn len(&self) -> usize {
        let mut ret: usize = 0;
        for pair in self.inner.clone() {
            ret += pair.1.len();
        }
        ret
    }
}

impl<'a, K, V> DoubleEndedIterator for MultiIter<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a V)> {
        loop {
            if let Some(a) = self.vec.as_mut() {
                if let Some(ret) = a.1.next_back() {
                    return Some((a.0, ret));
                }
            }
            match self.inner.next_back() {
                Some((a, b)) => {
                    self.vec = Some((a, b.iter()));
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }
}

pub struct MultiIterMut<'a, K, V> {
    vec: Option<(&'a K, std::slice::IterMut<'a, V>)>,
    inner: std::collections::btree_map::IterMut<'a, K, Vec<V>>,
}

impl<'a, K, V> Iterator for MultiIterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        loop {
            if let Some(a) = self.vec.as_mut() {
                if let Some(ret) = a.1.next() {
                    return Some((a.0, ret));
                }
            }
            match self.inner.next() {
                Some((a, b)) => {
                    self.vec = Some((a, b.iter_mut()));
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }

    fn last(mut self) -> Option<(&'a K, &'a mut V)> {
        self.next_back()
    }

    fn min(mut self) -> Option<(&'a K, &'a mut V)> {
        self.next()
    }

    fn max(mut self) -> Option<(&'a K, &'a mut V)> {
        self.next_back()
    }
}

impl<'a, K, V> DoubleEndedIterator for MultiIterMut<'a, K, V> {
    fn next_back(&mut self) -> Option<(&'a K, &'a mut V)> {
        loop {
            if let Some(a) = self.vec.as_mut() {
                if let Some(ret) = a.1.next_back() {
                    return Some((a.0, ret));
                }
            }
            match self.inner.next_back() {
                Some((a, b)) => {
                    self.vec = Some((a, b.iter_mut()));
                    continue;
                }
                None => {
                    return None;
                }
            }
        }
    }
}

#[macro_export]
/// Create a `BTreeMultiMap` from a list of key value pairs
macro_rules! btreemultimap{
    (@replace_with_unit $_t:tt) => { () };
    (@count $($key:expr),*) => { <[()]>::len(&[$($crate::btreemultimap! { @replace_with_unit $key }),*]) };

    ($($key:expr => $value:expr),* $(,)?)=>{
        {
            let mut map = $crate::BTreeMultiMap::new();
            $(
                map.insert($key,$value);
             )*
            map
        }
    }
}
