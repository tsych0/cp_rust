use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;

pub trait Itertools: Iterator {
    // Your existing methods...
    fn group_by<F, K>(self, key_fn: F) -> GroupBy<Self, F, K>
    where
        Self: Sized,
        F: Fn(&Self::Item) -> K,
        K: PartialEq,
    {
        GroupBy {
            iter: self.peekable(),
            key_fn,
        }
    }

    fn unique(self) -> Unique<Self>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        Unique::new(self)
    }

    // New methods
    fn is_sorted(self) -> bool
    where
        Self: Sized,
        Self::Item: PartialOrd,
    {
        let mut iter = self;
        let mut prev = match iter.next() {
            Some(item) => item,
            None => return true, // Empty iterator is sorted
        };

        for current in iter {
            if prev > current {
                return false;
            }
            prev = current;
        }
        true
    }

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

    fn sorted(self) -> Sorted<Self::Item>
    where
        Self: Sized,
        Self::Item: Ord,
    {
        let mut items: Vec<_> = self.collect();
        items.sort();
        Sorted::new(items)
    }

    fn sorted_by<F>(self, compare: F) -> Sorted<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item, &Self::Item) -> Ordering,
    {
        let mut items: Vec<_> = self.collect();
        items.sort_by(compare);
        Sorted::new(items)
    }

    fn sorted_by_key<F, K>(self, key_fn: F) -> Sorted<Self::Item>
    where
        Self: Sized,
        F: FnMut(&Self::Item) -> K,
        K: Ord,
    {
        let mut items: Vec<_> = self.collect();
        items.sort_by_key(key_fn);
        Sorted::new(items)
    }

    fn chunks(self, size: usize) -> Chunks<Self>
    where
        Self: Sized,
    {
        Chunks::new(self, size)
    }

    fn take_while_ref<P>(&mut self, predicate: P) -> TakeWhileRef<Self, P>
    where
        Self: Sized,
        P: FnMut(&Self::Item) -> bool,
    {
        TakeWhileRef::new(self, predicate)
    }

    fn intersperse(self, separator: Self::Item) -> Intersperse<Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        Intersperse::new(self, separator)
    }
}

// Sorted iterator wrapper
pub struct Sorted<T> {
    items: std::vec::IntoIter<T>,
}

impl<T> Sorted<T> {
    fn new(items: Vec<T>) -> Self {
        Self {
            items: items.into_iter(),
        }
    }
}

impl<T> Iterator for Sorted<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.items.next()
    }
}

// Chunks iterator
pub struct Chunks<I: Iterator> {
    iter: I,
    size: usize,
}

impl<I: Iterator> Chunks<I> {
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

// Intersperse iterator
pub struct Intersperse<I: Iterator> {
    iter: I,
    separator: I::Item,
    needs_separator: bool,
    pending: Option<I::Item>,
}

impl<I: Iterator> Intersperse<I>
where
    I::Item: Clone,
{
    fn new(mut iter: I, separator: I::Item) -> Self {
        let pending = iter.next();
        Self {
            iter,
            separator,
            needs_separator: false,
            pending,
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
            Some(self.separator.clone())
        } else if let Some(item) = self.pending.take() {
            self.pending = self.iter.next();
            if self.pending.is_some() {
                self.needs_separator = true;
            }
            Some(item)
        } else {
            None
        }
    }
}

// TakeWhileRef for borrowing predicate
pub struct TakeWhileRef<'a, I: Iterator, P> {
    iter: &'a mut I,
    predicate: P,
    done: bool,
}

impl<'a, I: Iterator, P> TakeWhileRef<'a, I, P> {
    fn new(iter: &'a mut I, predicate: P) -> Self {
        Self {
            iter,
            predicate,
            done: false,
        }
    }
}

impl<'a, I: Iterator, P> Iterator for TakeWhileRef<'a, I, P>
where
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        match self.iter.next() {
            Some(item) => {
                if (self.predicate)(&item) {
                    Some(item)
                } else {
                    self.done = true;
                    None
                }
            }
            None => None,
        }
    }
}

pub struct GroupBy<I, F, K>
where
    I: Iterator,
    F: Fn(&I::Item) -> K,
    K: PartialEq,
{
    iter: std::iter::Peekable<I>,
    key_fn: F,
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

pub struct Unique<I: Iterator> {
    iter: I,
    seen: HashSet<I::Item>,
}

impl<I> Unique<I>
where
    I: Iterator,
    I::Item: Eq + Hash + Clone,
{
    pub fn new(iter: I) -> Self {
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
        while let Some(v) = self.iter.next() {
            if self.seen.insert(v.clone()) {
                return Some(v);
            }
        }
        None
    }
}

impl<T: Iterator> Itertools for T {}

fn cartesian_product<I, J>(iter1: I, iter2: J) -> Vec<(I::Item, J::Item)>
where
    I: IntoIterator,
    J: IntoIterator,
    I::Item: Clone,
    J::Item: Clone,
{
    let iter1: Vec<_> = iter1.into_iter().collect();
    iter2
        .into_iter()
        .flat_map(|b| iter1.iter().map(move |a| (a.clone(), b.clone())))
        .collect()
}

fn permutations<I>(iterable: I, r: usize) -> Vec<Vec<I::Item>>
where
    I: IntoIterator,
    I::Item: Clone + PartialEq,
{
    let pool: Vec<_> = iterable.into_iter().collect();
    let n = pool.len();

    if r > n {
        return vec![];
    }

    let mut result = Vec::new();
    let mut indices: Vec<usize> = (0..n).collect();
    let mut cycles: Vec<usize> = (n - r + 1..=n).collect();

    // First permutation
    let perm: Vec<I::Item> = (0..r).map(|i| pool[indices[i]].clone()).collect();
    result.push(perm);

    while n > 0 {
        let mut found_valid = false;

        for i in (0..r).rev() {
            cycles[i] -= 1;
            if cycles[i] == 0 {
                // Move last element to position i
                indices.push(indices[i]);
                indices.remove(i);
                cycles[i] = n - i;
            } else {
                let j = cycles[i];
                indices.swap(i, n - j);

                // Add the next permutation
                let next_perm: Vec<I::Item> = (0..r).map(|i| pool[indices[i]].clone()).collect();
                result.push(next_perm);

                found_valid = true;
                break;
            }
        }

        if !found_valid {
            break;
        }
    }

    result
}
