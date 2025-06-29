//! Find minimum-spanning-tree is an undirected graph using
//! [Kruskal's algorithm](https://en.wikipedia.org/wiki/Kruskal's_algorithm).

use crate::indexmap::IndexSet;
use std::hash::Hash;
use std::mem;

// Find parent and compress path by path halving.
fn find(parents: &mut [usize], mut node: usize) -> usize {
    while parents[node] != node {
        parents[node] = parents[parents[node]];
        node = parents[node];
    }
    node
}

fn union(parents: &mut [usize], ranks: &mut [usize], mut a: usize, mut b: usize) {
    if ranks[a] < ranks[b] {
        mem::swap(&mut a, &mut b);
    }
    parents[b] = a;
    if ranks[a] == ranks[b] {
        ranks[a] += 1;
    }
}

/// Minimal-spanning-tree for nodes with integer indices. The nodes must have
/// consecutives indices between 0 and `number_of_nodes`-1.
pub fn kruskal_indices<C>(
    number_of_nodes: usize,
    edges: impl AsRef<[(usize, usize, C)]>,
) -> impl Iterator<Item = (usize, usize, C)>
where
    C: Clone + Ord,
{
    let mut parents = (0..number_of_nodes).collect::<Vec<_>>();
    let mut ranks = vec![1; number_of_nodes];
    let mut edges = edges.as_ref().to_vec();
    edges.sort_unstable_by(|a, b| a.2.cmp(&b.2));
    edges.into_iter().filter_map(move |(a, b, w)| {
        let ra = find(&mut parents, a);
        let rb = find(&mut parents, b);
        if ra == rb {
            None
        } else {
            union(&mut parents, &mut ranks, ra, rb);
            Some((a, b, w))
        }
    })
}

/// Find a minimum-spanning-tree. From a collection of
/// weighted edges, return an iterator of edges forming
/// a minimum-spanning-tree.
pub fn kruskal<N, C>(edges: &[(N, N, C)]) -> impl Iterator<Item = (&N, &N, C)>
where
    N: Hash + Eq,
    C: Clone + Ord,
{
    let mut nodes = IndexSet::new();
    let edges = edges
        .iter()
        .map(|(a, b, w)| {
            let ia = nodes.insert_full(a).0;
            let ib = nodes.insert_full(b).0;
            (ia, ib, w.clone())
        })
        .collect::<Vec<_>>();
    kruskal_indices(nodes.len(), edges).filter_map(move |(ia, ib, w)| {
        Some((
            <&N>::clone(nodes.get_index(ia)?), // Cannot fail
            <&N>::clone(nodes.get_index(ib)?), // Cannot fail
            w,
        ))
    })
}
