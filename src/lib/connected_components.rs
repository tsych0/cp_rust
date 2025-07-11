use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::marker::PhantomData;

/// A connected component implementation for various generic types.
pub struct ConnectedComponents<
    N,
    It = Vec<N>,
    It2 = HashSet<N>,
    C1 = HashSet<N>,
    C2 = Vec<C1>,
    C3 = HashMap<N, usize>,
> {
    _n: PhantomData<N>,
    _it: PhantomData<It>,
    _it2: PhantomData<It2>,
    _c1: PhantomData<C1>,
    _c2: PhantomData<C2>,
    _c3: PhantomData<C3>,
}

impl<N, It, It2, C1, C2, C3> ConnectedComponents<N, It, It2, C1, C2, C3>
where
    N: Hash + Eq + Clone,
    It: IntoIterator<Item = N> + Clone,
    for<'it> &'it It: IntoIterator<Item = &'it N>,
    for<'it> &'it It2: IntoIterator<Item = &'it N>,
    C1: FromIterator<N>,
    C2: FromIterator<C1>,
    C3: FromIterator<(N, usize)>,
{
    /// Separate components of an undirected graph into disjoint sets.
    pub fn separate_components(groups: &[It]) -> (HashMap<&N, usize>, Vec<usize>) {
        let mut table = (0..groups.len()).collect::<Vec<_>>();
        let mut indices = HashMap::new();
        for (mut group_index, group) in groups.iter().enumerate() {
            let mut is_empty = true;
            for element in group {
                is_empty = false;
                match indices.entry(element) {
                    Occupied(e) => {
                        table[group_index] = find(&mut table, *e.get());
                        group_index = table[group_index];
                    }
                    Vacant(e) => {
                        e.insert(group_index);
                    }
                }
            }
            if is_empty {
                table[group_index] = usize::MAX;
            }
        }
        for group_index in indices.values_mut() {
            *group_index = find(&mut table, *group_index);
        }
        for group_index in 0..groups.len() {
            if table[group_index] != usize::MAX {
                let target = find(&mut table, group_index);
                // Due to path halving, this particular entry might not
                // be up-to-date yet.
                table[group_index] = target;
            }
        }
        (indices, table)
    }

    /// Separate components of an undirected graph into disjoint sets.
    pub fn components(groups: &[It]) -> C2 {
        let (_, gindices) = Self::separate_components(groups);
        let mut gb: HashMap<usize, HashSet<N>> = HashMap::default();
        for (i, n) in gindices
            .into_iter()
            .enumerate()
            .filter(|&(_, n)| n != usize::MAX)
        {
            let set = gb.entry(n).or_default();
            for e in groups[i].clone() {
                set.insert(e);
            }
        }
        gb.into_values().map(|v| v.into_iter().collect()).collect()
    }

    /// Extract connected components from a graph.
    pub fn connected_components<FN, IN>(starts: &[N], mut neighbours: FN) -> C2
    where
        FN: FnMut(&N) -> IN,
        IN: IntoIterator<Item = N>,
    {
        ConnectedComponents::<N, Vec<N>, It2, C1, C2, C3>::components(
            &starts
                .iter()
                .map(|s| {
                    neighbours(s)
                        .into_iter()
                        .chain([s.clone()])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        )
    }

    /// Locate vertices amongst disjoint sets.
    pub fn component_index(components: &[It2]) -> C3 {
        components
            .iter()
            .enumerate()
            .flat_map(|(i, c)| c.into_iter().map(move |n| (n.clone(), i)))
            .collect()
    }
}

fn find(table: &mut [usize], mut x: usize) -> usize {
    while table[x] != x {
        let t = table[x];
        table[x] = table[table[x]];
        x = t;
    }
    x
}

/// Separate components of an undirected graph into disjoint sets.
pub fn separate_components<N>(groups: &[Vec<N>]) -> (HashMap<&N, usize>, Vec<usize>)
where
    N: Hash + Eq + Clone,
{
    ConnectedComponents::<N>::separate_components(groups)
}

/// Separate components of an undirected graph into disjoint sets.
pub fn components<N>(groups: &[Vec<N>]) -> Vec<HashSet<N>>
where
    N: Clone + Hash + Eq,
{
    ConnectedComponents::<N>::components(groups)
}

/// Extract connected components from a graph.
pub fn connected_components<N, FN, IN>(starts: &[N], neighbours: FN) -> Vec<HashSet<N>>
where
    N: Clone + Hash + Eq,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    ConnectedComponents::<N>::connected_components(starts, neighbours)
}

/// Locate vertices amongst disjoint sets.
pub fn component_index<N>(components: &[HashSet<N>]) -> HashMap<N, usize>
where
    N: Clone + Hash + Eq,
{
    ConnectedComponents::<N>::component_index(components)
}
