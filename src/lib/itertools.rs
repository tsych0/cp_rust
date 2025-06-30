use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::Peekable;

/// Extension trait providing additional iterator methods for competitive programming
pub trait Itertools: Iterator {
    /// Groups consecutive elements by a key function
    fn group_by<F, K>(self, key_fn: F) -> GroupBy<Self, F, K>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> K,
        K: PartialEq,
    {
        GroupBy::new(self, key_fn)
    }

    /// Removes duplicate elements while preserving order
    fn unique(self) -> Unique<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        Unique::new(self)
    }

    /// Checks if iterator is sorted in ascending order
    fn is_sorted(self) -> bool
    where
        Self: Sized,
        Self::Item: PartialOrd,
    {
        Itertools::is_sorted_by(self, |a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
    }

    /// Checks if iterator is sorted by comparison function
    fn is_sorted_by<F>(self, mut compare: F) -> bool
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut iter = self;
        let mut prev = match iter.next() {
            Some(item) => item,
            None => return true,
        };

        for current in iter {
            if compare(&prev, &current) == Ordering::Greater {
                return false;
            }
            prev = current;
        }
        true
    }

    /// Checks if iterator is sorted by key function
    fn is_sorted_by_key<F, K>(self, mut key_fn: F) -> bool
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> K,
        K: PartialOrd,
    {
        let mut iter = self;
        let mut prev_key = match iter.next() {
            Some(item) => key_fn(&item),
            None => return true,
        };

        for current in iter {
            let current_key = key_fn(&current);
            if prev_key > current_key {
                return false;
            }
            prev_key = current_key;
        }
        true
    }

    /// Returns sorted iterator (consumes original)
    fn sorted(self) -> Sorted<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut items: Vec<_> = self.collect();
        items.sort_unstable(); // Faster for CP
        Sorted::new(items)
    }

    /// Returns sorted iterator by comparison function
    fn sorted_by<F>(self, compare: F) -> Sorted<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut items: Vec<_> = self.collect();
        items.sort_unstable_by(compare);
        Sorted::new(items)
    }

    /// Returns sorted iterator by key function
    fn sorted_by_key<F, K>(self, key_fn: F) -> Sorted<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> K,
        K: Ord,
    {
        let mut items: Vec<_> = self.collect();
        items.sort_unstable_by_key(key_fn);
        Sorted::new(items)
    }

    /// Splits iterator into chunks of specified size
    fn chunks(self, size: usize) -> Chunks<Self>
    where
        Self: Sized,
    {
        Chunks::new(self, size)
    }

    /// Intersperses separator between elements
    fn intersperse(self, separator: Self::Item) -> Intersperse<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Intersperse::new(self, separator)
    }

    /// Collects into Vec (shorthand for competitive programming)
    fn collect_vec(self) -> Vec<Self::Item>
    where
        Self: Sized,
    {
        self.collect()
    }

    /// Finds minimum and maximum in one pass
    fn minmax(self) -> Option<(Self::Item, Self::Item)>
    where
        Self: Sized,
        Self::Item: Ord + Clone,
    {
        let mut iter = self;
        let first = iter.next()?;
        let (mut min, mut max) = (first.clone(), first);

        for item in iter {
            if item < min {
                min = item.clone();
            }
            if item > max {
                max = item;
            }
        }
        Some((min, max))
    }

    /// Counts elements satisfying predicate
    fn count_where<P>(self, mut predicate: P) -> usize
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        self.filter(|x| predicate(x)).count()
    }

    /// Creates frequency map
    fn frequencies(self) -> std::collections::HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Eq + Hash,
    {
        let mut freq = std::collections::HashMap::new();
        for item in self {
            *freq.entry(item).or_insert(0) += 1;
        }
        freq
    }
}

/// Sorted iterator wrapper
pub struct Sorted<T> {
    items: std::vec::IntoIter<T>,
}

impl<T> Sorted<T> {
    #[inline]
    fn new(items: Vec<T>) -> Self {
        Self {
            items: items.into_iter(),
        }
    }
}

impl<T> Iterator for Sorted<T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.items.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.items.size_hint()
    }
}

impl<T> ExactSizeIterator for Sorted<T> {}

/// Chunks iterator
pub struct Chunks<I: Iterator> {
    iter: I,
    size: usize,
}

impl<I: Iterator> Chunks<I> {
    #[inline]
    fn new(iter: I, size: usize) -> Self {
        assert!(size > 0, "Chunk size must be greater than 0");
        Self { iter, size }
    }
}

impl<I: Iterator> Iterator for Chunks<I> {
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut chunk = Vec::with_capacity(self.size);
        for _ in 0..self.size {
            match self.iter.next() {
                Some(item) => chunk.push(item),
                None => break,
            }
        }
        if chunk.is_empty() {
            None
        } else {
            Some(chunk)
        }
    }
}

/// Intersperse iterator
pub struct Intersperse<I: Iterator> {
    iter: Peekable<I>,
    separator: I::Item,
    needs_separator: bool,
}

impl<I: Iterator> Intersperse<I>
where
    I::Item: Clone,
{
    #[inline]
    fn new(iter: I, separator: I::Item) -> Self {
        Self {
            iter: iter.peekable(),
            separator,
            needs_separator: false,
        }
    }
}

impl<I: Iterator> Iterator for Intersperse<I>
where
    I::Item: Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.needs_separator {
            self.needs_separator = false;
            return Some(self.separator.clone());
        }

        match self.iter.next() {
            Some(item) => {
                if self.iter.peek().is_some() {
                    self.needs_separator = true;
                }
                Some(item)
            }
            None => None,
        }
    }
}

/// GroupBy iterator
pub struct GroupBy<I, F, K>
where
    I: Iterator,
    F: Fn(&I::Item) -> K,
    K: PartialEq,
{
    iter: Peekable<I>,
    key_fn: F,
}

impl<I, F, K> GroupBy<I, F, K>
where
    I: Iterator,
    F: Fn(&I::Item) -> K,
    K: PartialEq,
{
    #[inline]
    fn new(iter: I, key_fn: F) -> Self {
        Self {
            iter: iter.peekable(),
            key_fn,
        }
    }
}

impl<I, F, K> Iterator for GroupBy<I, F, K>
where
    I: Iterator,
    F: Fn(&I::Item) -> K,
    K: PartialEq,
{
    type Item = Vec<I::Item>;

    fn next(&mut self) -> Option<Self::Item> {
        let first = self.iter.next()?;
        let current_key = (self.key_fn)(&first);
        let mut group = vec![first];

        while let Some(peeked) = self.iter.peek() {
            if (self.key_fn)(peeked) == current_key {
                group.push(self.iter.next().unwrap());
            } else {
                break;
            }
        }
        Some(group)
    }
}

/// Unique iterator
pub struct Unique<I: Iterator> {
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I> Unique<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    #[inline]
    fn new(iter: I) -> Self {
        Self {
            iter,
            seen: HashSet::new(),
        }
    }
}

impl<I> Iterator for Unique<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let item = self.iter.next()?;
            if self.seen.insert(item.clone()) {
                return Some(item);
            }
        }
    }
}

// Implement Itertools for all iterators
impl<T: Iterator> Itertools for T {}

/// Cartesian product of two iterables
pub fn cartesian_product<I, J>(iter1: I, iter2: J) -> Vec<(I::Item, J::Item)>
where
    I: IntoIterator,
    J: IntoIterator,
    I::Item: Clone,
    J::Item: Clone,
{
    let vec1: Vec<_> = iter1.into_iter().collect();
    let vec2: Vec<_> = iter2.into_iter().collect();

    let mut result = Vec::with_capacity(vec1.len() * vec2.len());
    for b in &vec2 {
        for a in &vec1 {
            result.push((a.clone(), b.clone()));
        }
    }
    result
}

/// Generate all permutations of length r
pub fn permutations<I>(iterable: I, r: usize) -> Vec<Vec<I::Item>>
where
    I: IntoIterator,
    I::Item: Clone,
{
    let pool: Vec<_> = iterable.into_iter().collect();
    let n = pool.len();

    if r > n || r == 0 {
        return if r == 0 { vec![vec![]] } else { vec![] };
    }

    let mut result = Vec::new();
    generate_permutations(&pool, r, &mut Vec::new(), &mut vec![false; n], &mut result);
    result
}

fn generate_permutations<T: Clone>(
    pool: &[T],
    r: usize,
    current: &mut Vec<T>,
    used: &mut [bool],
    result: &mut Vec<Vec<T>>,
) {
    if current.len() == r {
        result.push(current.clone());
        return;
    }

    for i in 0..pool.len() {
        if !used[i] {
            used[i] = true;
            current.push(pool[i].clone());
            generate_permutations(pool, r, current, used, result);
            current.pop();
            used[i] = false;
        }
    }
}

/// Generate all combinations of length r
pub fn combinations<I>(iterable: I, r: usize) -> Vec<Vec<I::Item>>
where
    I: IntoIterator,
    I::Item: Clone,
{
    let pool: Vec<_> = iterable.into_iter().collect();
    let n = pool.len();

    if r > n {
        return vec![];
    }
    if r == 0 {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    generate_combinations(&pool, r, 0, &mut Vec::new(), &mut result);
    result
}

fn generate_combinations<T: Clone>(
    pool: &[T],
    r: usize,
    start: usize,
    current: &mut Vec<T>,
    result: &mut Vec<Vec<T>>,
) {
    if current.len() == r {
        result.push(current.clone());
        return;
    }

    for i in start..pool.len() {
        current.push(pool[i].clone());
        generate_combinations(pool, r, i + 1, current, result);
        current.pop();
    }
}
