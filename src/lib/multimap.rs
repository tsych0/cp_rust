use std::borrow::Borrow;
use std::collections::hash_map::{IntoIter, Keys, RandomState};
use std::collections::HashMap;
use std::fmt::{self, Debug};
use std::hash::{BuildHasher, Hash};
use std::iter::{FromIterator, IntoIterator, Iterator};
use std::ops::Index;

pub use std::collections::hash_map::Iter as IterAll;
pub use std::collections::hash_map::IterMut as IterAllMut;

pub use entry::{Entry, OccupiedEntry, VacantEntry};

mod entry {
    use std::collections::hash_map::OccupiedEntry as HashMapOccupiedEntry;
    use std::collections::hash_map::VacantEntry as HashMapVacantEntry;

    /// A view into a single occupied location in a MultiMap.
    pub struct OccupiedEntry<'a, K: 'a, V: 'a> {
        #[doc(hidden)]
        pub inner: HashMapOccupiedEntry<'a, K, Vec<V>>,
    }

    /// A view into a single empty location in a MultiMap.
    pub struct VacantEntry<'a, K: 'a, V: 'a> {
        #[doc(hidden)]
        pub inner: HashMapVacantEntry<'a, K, Vec<V>>,
    }

    /// A view into a single location in a map, which may be vacant or occupied.
    pub enum Entry<'a, K: 'a, V: 'a> {
        /// An occupied Entry.
        Occupied(OccupiedEntry<'a, K, V>),

        /// A vacant Entry.
        Vacant(VacantEntry<'a, K, V>),
    }

    impl<'a, K: 'a, V: 'a> OccupiedEntry<'a, K, V> {
        /// Gets a reference to the first item in value in the vector corresponding to entry.
        ///
        /// # Panics
        ///
        /// This method will panic if the key has zero values.
        pub fn get(&self) -> &V {
            self.inner.get().first().expect("no values in entry")
        }

        /// Gets a reference to the values (vector) corresponding to entry.
        pub fn get_vec(&self) -> &Vec<V> {
            self.inner.get()
        }

        /// Gets a mut reference to the first item in value in the vector corresponding to entry.
        ///
        /// # Panics
        ///
        /// This method will panic if the key has zero values.
        pub fn get_mut(&mut self) -> &mut V {
            self.inner
                .get_mut()
                .first_mut()
                .expect("no values in entry")
        }

        /// Gets a mut reference to the values (vector) corresponding to entry.
        pub fn get_vec_mut(&mut self) -> &mut Vec<V> {
            self.inner.get_mut()
        }

        /// Converts the OccupiedEntry into a mutable reference to the first item in value in the entry
        /// with a lifetime bound to the map itself
        pub fn into_mut(self) -> &'a mut V {
            &mut self.inner.into_mut()[0]
        }

        /// Converts the OccupiedEntry into a mutable reference to the values (vector) in the entry
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

    impl<'a, K: 'a, V: 'a> VacantEntry<'a, K, V> {
        /// Sets the first value in the vector of the entry with the VacantEntry's key,
        /// and returns a mutable reference to it.
        pub fn insert(self, value: V) -> &'a mut V {
            &mut self.inner.insert(vec![value])[0]
        }

        /// Sets values in the entry with the VacantEntry's key,
        /// and returns a mutable reference to it.
        pub fn insert_vec(self, values: Vec<V>) -> &'a mut Vec<V> {
            self.inner.insert(values)
        }
    }

    impl<'a, K: 'a, V: 'a> Entry<'a, K, V> {
        /// Ensures a value is in the entry by inserting the default if empty, and returns
        /// a mutable reference to the value in the entry. This will return a mutable reference to the
        /// first value in the vector corresponding to the specified key.
        pub fn or_insert(self, default: V) -> &'a mut V {
            match self {
                Entry::Occupied(entry) => entry.into_mut(),
                Entry::Vacant(entry) => entry.insert(default),
            }
        }

        /// Ensures a value is in the entry by inserting the default values if empty, and returns
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
pub struct MultiMap<K, V, S = RandomState> {
    inner: HashMap<K, Vec<V>, S>,
}

impl<K, V> MultiMap<K, V>
where
    K: Eq + Hash,
{
    /// Creates an empty MultiMap
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map: MultiMap<&str, isize> = MultiMap::new();
    /// ```
    pub fn new() -> MultiMap<K, V> {
        MultiMap {
            inner: HashMap::new(),
        }
    }

    /// Creates an empty multimap with the given initial capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map: MultiMap<&str, isize> = MultiMap::with_capacity(20);
    /// ```
    pub fn with_capacity(capacity: usize) -> MultiMap<K, V> {
        MultiMap {
            inner: HashMap::with_capacity(capacity),
        }
    }
}

impl<K, V, S> MultiMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    /// Creates an empty MultiMap which will use the given hash builder to hash keys.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let s = RandomState::new();
    /// let mut map: MultiMap<&str, isize> = MultiMap::with_hasher(s);
    /// ```
    pub fn with_hasher(hash_builder: S) -> MultiMap<K, V, S> {
        MultiMap {
            inner: HashMap::with_hasher(hash_builder),
        }
    }

    /// Creates an empty MultiMap with the given intial capacity and hash builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    /// use std::collections::hash_map::RandomState;
    ///
    /// let s = RandomState::new();
    /// let mut map: MultiMap<&str, isize> = MultiMap::with_capacity_and_hasher(20, s);
    /// ```
    pub fn with_capacity_and_hasher(capacity: usize, hash_builder: S) -> MultiMap<K, V, S> {
        MultiMap {
            inner: HashMap::with_capacity_and_hasher(capacity, hash_builder),
        }
    }

    /// Inserts a key-value pair into the multimap. If the key does exist in
    /// the map then the value is pushed to that key's vector. If the key doesn't
    /// exist in the map a new vector with the given value is inserted.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert("key", 42);
    /// ```
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

    /// Inserts multiple key-value pairs into the multimap. If the key does exist in
    /// the map then the values are extended into that key's vector. If the key
    /// doesn't exist in the map a new vector collected from the given values is inserted.
    ///
    /// This may be more efficient than inserting values independently.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::<&str, &usize>::new();
    /// map.insert_many("key", &[42, 43]);
    /// ```
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

    /// Inserts multiple key-value pairs into the multimap. If the key does exist in
    /// the map then the values are extended into that key's vector. If the key
    /// doesn't exist in the map a new vector collected from the given values is inserted.
    ///
    /// This may be more efficient than inserting values independently.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::<&str, usize>::new();
    /// map.insert_many_from_slice("key", &[42, 43]);
    /// ```
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
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// assert_eq!(map.contains_key(&1), true);
    /// assert_eq!(map.contains_key(&2), false);
    /// ```
    pub fn contains_key<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.contains_key(k)
    }

    /// Returns the number of unique keys in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(2, 1337);
    /// map.insert(2, 31337);
    /// assert_eq!(map.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Removes a key from the map, returning the vector of values at
    /// the key if the key was previously in the map.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// assert_eq!(map.remove(&1), Some(vec![42, 1337]));
    /// assert_eq!(map.remove(&1), None);
    /// ```
    pub fn remove<Q>(&mut self, k: &Q) -> Option<Vec<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.remove(k)
    }

    /// Returns a reference to the first item in the vector corresponding to
    /// the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// assert_eq!(map.get(&1), Some(&42));
    /// ```
    pub fn get<Q>(&self, k: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get(k)?.first()
    }

    /// Returns a mutable reference to the first item in the vector corresponding to
    /// the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// if let Some(v) = map.get_mut(&1) {
    ///     *v = 99;
    /// }
    /// assert_eq!(map[&1], 99);
    /// ```
    pub fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get_mut(k)?.get_mut(0)
    }

    /// Returns a reference to the vector corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// assert_eq!(map.get_vec(&1), Some(&vec![42, 1337]));
    /// ```
    pub fn get_vec<Q>(&self, k: &Q) -> Option<&Vec<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get(k)
    }

    /// Returns a mutable reference to the vector corresponding to the key.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// if let Some(v) = map.get_vec_mut(&1) {
    ///     (*v)[0] = 1991;
    ///     (*v)[1] = 2332;
    /// }
    /// assert_eq!(map.get_vec(&1), Some(&vec![1991, 2332]));
    /// ```
    pub fn get_vec_mut<Q>(&mut self, k: &Q) -> Option<&mut Vec<V>>
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        self.inner.get_mut(k)
    }

    /// Returns true if the key is multi-valued.
    ///
    /// The key may be any borrowed form of the map's key type, but Hash and Eq
    /// on the borrowed form must match those for the key type.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1, 42);
    /// map.insert(1, 1337);
    /// map.insert(2, 2332);
    ///
    /// assert_eq!(map.is_vec(&1), true);   // key is multi-valued
    /// assert_eq!(map.is_vec(&2), false);  // key is single-valued
    /// assert_eq!(map.is_vec(&3), false);  // key not in map
    /// ```
    pub fn is_vec<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Eq + Hash + ?Sized,
    {
        match self.get_vec(k) {
            Some(val) => val.len() > 1,
            None => false,
        }
    }

    /// Returns the number of elements the map can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let map: MultiMap<usize, usize> = MultiMap::new();
    /// assert!(map.capacity() >= 0);
    /// ```
    pub fn capacity(&self) -> usize {
        self.inner.capacity()
    }

    /// Returns true if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// assert!(map.is_empty());
    /// map.insert(1,42);
    /// assert!(!map.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Clears the map, removing all key-value pairs.
    /// Keeps the allocated memory for reuse.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.inner.clear();
    }

    /// An iterator visiting all keys in arbitrary order.
    /// Iterator element type is &'a K.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(2,1337);
    /// map.insert(4,1991);
    ///
    /// let mut keys: Vec<_> = map.keys().collect();
    /// keys.sort();
    /// assert_eq!(keys, [&1, &2, &4]);
    /// ```
    pub fn keys(&'_ self) -> Keys<'_, K, Vec<V>> {
        self.inner.keys()
    }

    /// An iterator visiting pairs of each key and its first value in arbitrary order.
    /// The iterator returns
    /// a reference to the key and the first element in the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    ///
    /// See [`flat_iter`](Self::flat_iter)
    /// for visiting all key-value pairs,
    /// or [`iter_all`](Self::iter_all)
    /// for visiting each key and its vector of values.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// let mut pairs: Vec<_> = map.iter().collect();
    /// pairs.sort_by_key(|p| p.0);
    /// assert_eq!(pairs, [(&1, &42), (&3, &2332), (&4, &1991)]);
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            inner: self.inner.iter(),
        }
    }

    /// A mutable iterator visiting pairs of each key and its first value
    /// in arbitrary order. The iterator returns
    /// a reference to the key and a mutable reference to the first element in the
    /// corresponding key's vector. Iterator element type is (&'a K, &'a mut V).
    ///
    /// See [`flat_iter_mut`](Self::flat_iter_mut)
    /// for visiting all key-value pairs,
    /// or [`iter_all_mut`](Self::iter_all_mut)
    /// for visiting each key and its vector of values.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (_, value) in map.iter_mut() {
    ///     *value *= *value;
    /// }
    ///
    /// let mut pairs: Vec<_> = map.iter_mut().collect();
    /// pairs.sort_by_key(|p| p.0);
    /// assert_eq!(pairs, [(&1, &mut 1764), (&3, &mut 5438224), (&4, &mut 3964081)]);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        IterMut {
            inner: self.inner.iter_mut(),
        }
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// let mut pairs: Vec<_> = map.iter_all().collect();
    /// pairs.sort_by_key(|p| p.0);
    /// assert_eq!(pairs, [(&1, &vec![42, 1337]), (&3, &vec![2332]), (&4, &vec![1991])]);
    /// ```
    pub fn iter_all(&self) -> IterAll<K, Vec<V>> {
        self.inner.iter()
    }

    /// An iterator visiting all key-value pairs in arbitrary order. The iterator returns
    /// a reference to the key and the corresponding key's vector.
    /// Iterator element type is (&'a K, &'a V).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (key, values) in map.iter_all_mut() {
    ///     for value in values.iter_mut() {
    ///         *value = 99;
    ///     }
    /// }
    ///
    /// let mut pairs: Vec<_> = map.iter_all_mut().collect();
    /// pairs.sort_by_key(|p| p.0);
    /// assert_eq!(pairs, [(&1, &mut vec![99, 99]), (&3, &mut vec![99]), (&4, &mut vec![99])]);
    /// ```
    pub fn iter_all_mut(&mut self) -> IterAllMut<K, Vec<V>> {
        self.inner.iter_mut()
    }

    /// An iterator visiting all key-value pairs in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// let mut pairs: Vec<_> = map.flat_iter().collect();
    /// pairs.sort();
    /// assert_eq!(pairs, [(&1, &42), (&1, &1337), (&3, &2332), (&4, &1991)]);
    /// ```
    pub fn flat_iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.iter_all()
            .flat_map(|(k, v)| v.iter().map(move |i| (k, i)))
    }

    /// A mutable iterator visiting all key-value pairs in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut map = MultiMap::new();
    /// map.insert(1,42);
    /// map.insert(1,1337);
    /// map.insert(3,2332);
    /// map.insert(4,1991);
    ///
    /// for (key, value) in map.flat_iter_mut() {
    ///     *value *= key;
    /// }
    ///
    /// let mut pairs: Vec<_> = map.flat_iter().collect();
    /// pairs.sort();
    /// assert_eq!(pairs, [(&1, &42), (&1, &1337), (&3, &6996), (&4, &7964)]);
    /// ```
    pub fn flat_iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.iter_all_mut()
            .flat_map(|(k, v)| v.iter_mut().map(move |i| (k, i)))
    }

    /// Gets the specified key's corresponding entry in the map for in-place manipulation.
    /// It's possible to both manipulate the vector and the 'value' (the first value in the
    /// vector).
    ///
    /// # Examples
    ///
    /// ```
    /// use multimap::MultiMap;
    ///
    /// let mut m = MultiMap::new();
    /// m.insert(1, 42);
    ///
    /// {
    ///     let mut v = m.entry(1).or_insert(43);
    ///     assert_eq!(v, &42);
    ///     *v = 44;
    /// }
    /// assert_eq!(m.entry(2).or_insert(666), &666);
    ///
    /// {
    ///     let mut v = m.entry(1).or_insert_vec(vec![43]);
    ///     assert_eq!(v, &vec![44]);
    ///     v.push(50);
    /// }
    /// assert_eq!(m.entry(2).or_insert_vec(vec![667]), &vec![666]);
    ///
    /// assert_eq!(m.get_vec(&1), Some(&vec![44, 50]));
    /// ```
    pub fn entry(&mut self, k: K) -> Entry<K, V> {
        use std::collections::hash_map::Entry as HashMapEntry;
        match self.inner.entry(k) {
            HashMapEntry::Occupied(entry) => Entry::Occupied(OccupiedEntry { inner: entry }),
            HashMapEntry::Vacant(entry) => Entry::Vacant(VacantEntry { inner: entry }),
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all pairs `(k, v)` such that `f(&k,&mut v)` returns `false`.
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&K, &V) -> bool,
    {
        for (key, vector) in &mut self.inner {
            vector.retain(|value| f(key, value));
        }
        self.inner.retain(|_, v| !v.is_empty());
    }
}

impl<K, V, S, Q> Index<&Q> for MultiMap<K, V, S>
where
    K: Eq + Hash + Borrow<Q>,
    Q: Eq + Hash + ?Sized,
    S: BuildHasher,
{
    type Output = V;

    fn index(&self, index: &Q) -> &V {
        self.inner
            .get(index)
            .expect("no entry found for key")
            .first()
            .expect("no value found for key")
    }
}

impl<K, V, S> Debug for MultiMap<K, V, S>
where
    K: Eq + Hash + Debug,
    V: Debug,
    S: BuildHasher,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.iter_all()).finish()
    }
}

impl<K, V, S> PartialEq for MultiMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn eq(&self, other: &MultiMap<K, V, S>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        self.iter_all()
            .all(|(key, value)| other.get_vec(key).is_some_and(|v| *value == *v))
    }
}

impl<K, V, S> Eq for MultiMap<K, V, S>
where
    K: Eq + Hash,
    V: Eq,
    S: BuildHasher,
{
}

impl<K, V, S> Default for MultiMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn default() -> MultiMap<K, V, S> {
        MultiMap {
            inner: Default::default(),
        }
    }
}

impl<K, V, S> FromIterator<(K, V)> for MultiMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iterable: T) -> MultiMap<K, V, S> {
        let iter = iterable.into_iter();
        let hint = iter.size_hint().0;

        let mut multimap = MultiMap::with_capacity_and_hasher(hint, S::default());
        for (k, v) in iter {
            multimap.insert(k, v);
        }

        multimap
    }
}

impl<K, V, S> FromIterator<(K, Vec<V>)> for MultiMap<K, V, S>
where
    K: Eq + Hash,
    V: Clone,
    S: BuildHasher + Default,
{
    fn from_iter<T: IntoIterator<Item = (K, Vec<V>)>>(iterable: T) -> MultiMap<K, V, S> {
        let iter = iterable.into_iter();
        let hint = iter.size_hint().0;

        let mut multimap = MultiMap::with_capacity_and_hasher(hint, S::default());
        for (k, v) in iter {
            multimap.insert_many_from_slice(k, &v[..])
        }

        multimap
    }
}

impl<'a, K, V, S> IntoIterator for &'a MultiMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a K, &'a Vec<V>);
    type IntoIter = IterAll<'a, K, Vec<V>>;

    fn into_iter(self) -> IterAll<'a, K, Vec<V>> {
        self.iter_all()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut MultiMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (&'a K, &'a mut Vec<V>);
    type IntoIter = IterAllMut<'a, K, Vec<V>>;

    fn into_iter(self) -> IterAllMut<'a, K, Vec<V>> {
        self.inner.iter_mut()
    }
}

impl<K, V, S> IntoIterator for MultiMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    type Item = (K, Vec<V>);
    type IntoIter = IntoIter<K, Vec<V>>;

    fn into_iter(self) -> IntoIter<K, Vec<V>> {
        self.inner.into_iter()
    }
}

impl<K, V, S> Extend<(K, V)> for MultiMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<'a, K, V, S> Extend<(&'a K, &'a V)> for MultiMap<K, V, S>
where
    K: Eq + Hash + Copy,
    V: Copy,
    S: BuildHasher,
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a V)>>(&mut self, iter: T) {
        self.extend(iter.into_iter().map(|(&key, &value)| (key, value)));
    }
}

impl<K, V, S> Extend<(K, Vec<V>)> for MultiMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
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

impl<'a, K, V, S> Extend<(&'a K, &'a Vec<V>)> for MultiMap<K, V, S>
where
    K: Eq + Hash + Copy,
    V: Copy,
    S: BuildHasher,
{
    fn extend<T: IntoIterator<Item = (&'a K, &'a Vec<V>)>>(&mut self, iter: T) {
        self.extend(
            iter.into_iter()
                .map(|(&key, values)| (key, values.to_owned())),
        );
    }
}

#[derive(Clone)]
pub struct Iter<'a, K: 'a, V: 'a> {
    inner: IterAll<'a, K, Vec<V>>,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
        let (k, v) = self.inner.next()?;
        let v = v.first()?;
        Some((k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<K, V> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

pub struct IterMut<'a, K: 'a, V: 'a> {
    inner: IterAllMut<'a, K, Vec<V>>,
}

impl<'a, K, V> Iterator for IterMut<'a, K, V> {
    type Item = (&'a K, &'a mut V);

    fn next(&mut self) -> Option<(&'a K, &'a mut V)> {
        let (k, v) = self.inner.next()?;
        let v = v.first_mut()?;
        Some((k, v))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<K, V> ExactSizeIterator for IterMut<'_, K, V> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[macro_export]
/// Create a `MultiMap` from a list of key value pairs
macro_rules! multimap{
    (@replace_with_unit $_t:tt) => { () };
    (@count $($key:expr),*) => { <[()]>::len(&[$($crate::multimap! { @replace_with_unit $key }),*]) };

    ($($key:expr => $value:expr),* $(,)?)=>{
        {
            let mut map = $crate::MultiMap::with_capacity($crate::multimap! { @count $($key),* });
            $(
                map.insert($key,$value);
             )*
            map
        }
    }
}
