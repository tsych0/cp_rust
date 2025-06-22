use std::collections::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::ops::Deref;

/// A set of node references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeRefs<'a, N>(HashSet<&'a N>)
where
    N: Eq + Hash + Clone;

impl<'a, N: Eq + Hash + Clone> FromIterator<&'a N> for NodeRefs<'a, N> {
    fn from_iter<T: IntoIterator<Item = &'a N>>(iter: T) -> Self {
        NodeRefs(HashSet::from_iter(iter))
    }
}

impl<'a, N: Eq + Hash + Clone> From<&'a N> for NodeRefs<'a, N> {
    fn from(value: &'a N) -> Self {
        NodeRefs(HashSet::from_iter([value]))
    }
}

impl<'a, N: Eq + Hash + Clone> IntoIterator for NodeRefs<'a, N> {
    type Item = &'a N;
    type IntoIter = std::collections::hash_set::IntoIter<&'a N>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, N: Eq + Hash + Clone> IntoIterator for &'a NodeRefs<'a, N> {
    type Item = &'a N;
    type IntoIter = std::iter::Copied<std::collections::hash_set::Iter<'a, &'a N>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().copied()
    }
}

impl<'a, N: Eq + Hash + Clone> Deref for NodeRefs<'a, N> {
    type Target = HashSet<&'a N>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
