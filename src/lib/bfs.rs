//! Compute a shortest path using the [breadth-first search
//! algorithm](https://en.wikipedia.org/wiki/Breadth-first_search).

use crate::indexmap::{Entry::Vacant, IndexMap, IndexSet};
use crate::noderefs::NodeRefs;
use crate::pathfinding::reverse_path;
use std::hash::Hash;
use std::iter::FusedIterator;

/// Compute a shortest path using the [breadth-first search
/// algorithm](https://en.wikipedia.org/wiki/Breadth-first_search).
pub fn bfs<'a, N, S, FN, IN, FS>(start: S, successors: FN, success: FS) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone + 'a,
    S: Into<NodeRefs<'a, N>>,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    bfs_core(&start.into(), successors, success, true)
}

fn bfs_core<'a, N, FN, IN, FS>(
    start: &NodeRefs<'a, N>,
    mut successors: FN,
    mut success: FS,
    check_first: bool,
) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone + 'a,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool,
{
    if check_first {
        for start_node in start {
            if success(start_node) {
                return Some(vec![start_node.clone()]);
            }
        }
    }

    let mut parents: IndexMap<N, usize> = IndexMap::new();
    parents.extend(start.into_iter().map(|n| (n.clone(), usize::MAX)));

    let mut i = 0;
    while let Some((node, _)) = parents.get_index(i) {
        for successor in successors(node) {
            if success(&successor) {
                let mut path = reverse_path(&parents, |&p| p, i);
                path.push(successor);
                return Some(path);
            }
            if let Vacant(e) = parents.entry(successor) {
                e.insert(i);
            }
        }
        i += 1;
    }
    None
}

/// Return one of the shortest loop from start to start if it exists, `None` otherwise.
pub fn bfs_loop<'a, N, S, FN, IN>(start: S, successors: FN) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone + 'a,
    S: Into<NodeRefs<'a, N>>,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let start = start.into();
    bfs_core(&start, successors, |n| start.contains(n), false)
}

/// Compute a shortest path using the [breadth-first search
/// algorithm](https://en.wikipedia.org/wiki/Breadth-first_search) with
/// [bidirectional search].
#[allow(clippy::missing_panics_doc)]
pub fn bfs_bidirectional<'a, N, S, E, FNS, FNP, IN>(
    start: S,
    end: E,
    successors_fn: FNS,
    predecessors_fn: FNP,
) -> Option<Vec<N>>
where
    N: Eq + Hash + Clone + 'a,
    E: Into<NodeRefs<'a, N>>,
    S: Into<NodeRefs<'a, N>>,
    FNS: Fn(&N) -> IN,
    FNP: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let start = start.into();
    let end = end.into();

    let mut predecessors: IndexMap<N, Option<usize>> = IndexMap::new();
    predecessors.extend(start.into_iter().cloned().map(|n| (n, None)));
    let mut successors: IndexMap<N, Option<usize>> = IndexMap::new();
    successors.extend(end.into_iter().cloned().map(|n| (n, None)));

    let mut i_forwards = 0;
    let mut i_backwards = 0;
    let middle = 'l: loop {
        for _ in 0..(predecessors.len() - i_forwards) {
            let node = predecessors.get_index(i_forwards).unwrap().0;
            for successor_node in successors_fn(node) {
                if !predecessors.contains_key(&successor_node) {
                    predecessors.insert(successor_node.clone(), Some(i_forwards));
                }
                if successors.contains_key(&successor_node) {
                    break 'l Some(successor_node);
                }
            }
            i_forwards += 1;
        }

        for _ in 0..(successors.len() - i_backwards) {
            let node = successors.get_index(i_backwards).unwrap().0;
            for predecessor_node in predecessors_fn(node) {
                if !successors.contains_key(&predecessor_node) {
                    successors.insert(predecessor_node.clone(), Some(i_backwards));
                }
                if predecessors.contains_key(&predecessor_node) {
                    break 'l Some(predecessor_node);
                }
            }
            i_backwards += 1;
        }

        if i_forwards == predecessors.len() && i_backwards == successors.len() {
            break 'l None;
        }
    };

    middle.map(|middle| {
        let mut path = vec![];
        let mut node = Some(middle.clone());
        while let Some(n) = node {
            path.push(n.clone());
            node = predecessors[&n].map(|i| predecessors.get_index(i).unwrap().0.clone());
        }
        path.reverse();
        let mut node = successors[&middle].map(|i| successors.get_index(i).unwrap().0.clone());
        while let Some(n) = node {
            path.push(n.clone());
            node = successors[&n].map(|i| successors.get_index(i).unwrap().0.clone());
        }
        path
    })
}

/// Visit all nodes that are reachable from a start node. The node will be visited
/// in BFS order, starting from the `start` node and following the order returned
/// by the `successors` function.
pub fn bfs_reach<N, FN, IN>(start: N, successors: FN) -> BfsReachable<N, FN>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    let mut seen = IndexSet::new();
    seen.insert(start);
    BfsReachable {
        i: 0,
        seen,
        successors,
    }
}

/// Struct returned by [`bfs_reach`].
pub struct BfsReachable<N, FN> {
    i: usize,
    seen: IndexSet<N>,
    successors: FN,
}

impl<N, FN> BfsReachable<N, FN> {
    /// Return a lower bound on the number of remaining reachable
    /// nodes. Not all nodes are necessarily known in advance, and
    /// new reachable nodes may be discovered while using the iterator.
    pub fn remaining_nodes_low_bound(&self) -> usize {
        self.seen.len() - self.i
    }
}

impl<N, FN, IN> Iterator for BfsReachable<N, FN>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.seen.get_index(self.i)?.clone();
        for s in (self.successors)(&n) {
            self.seen.insert(s);
        }
        self.i += 1;
        Some(n)
    }
}

impl<N, FN, IN> FusedIterator for BfsReachable<N, FN>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
{
}
