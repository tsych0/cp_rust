use std::collections::HashSet;
use std::hash::Hash;

pub trait Itertools: Iterator {
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
    let mut perm: Vec<I::Item> = (0..r).map(|i| pool[indices[i]].clone()).collect();
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
