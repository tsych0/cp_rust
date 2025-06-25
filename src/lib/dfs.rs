use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FusedIterator;

/// Compute a path using the [depth-first search algorithm](https://en.wikipedia.org/wiki/Depth-first_search).
pub fn dfs<N, FN, IN, FS>(start: N, mut successors: FN, mut success: FS) -> Option<Vec<N>>
where
    N: Clone + Eq + Hash,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>,
    FS: FnMut(&N) -> bool, {
    let mut to_visit = vec![start];
    let mut visited: HashSet<N> = HashSet::default();
    let mut parents = HashMap::default();
    while let Some(node) = to_visit.pop() {
        if visited.insert(node.clone()) {
            if success(&node) {
                return Some(build_path(node, &parents));
            }
            for next in successors(&node)
                .into_iter()
                .collect::<Vec<_>>()
                .into_iter()
                .rev() {
                if !visited.contains(&next) {
                    parents.insert(next.clone(), node.clone());
                    to_visit.push(next);
                }
            }
        }
    }
    None
}

fn build_path<N>(mut node: N, parents: &HashMap<N, N>) -> Vec<N>
where
    N: Clone + Eq + Hash, {
    let mut path = vec![node.clone()];
    while let Some(parent) = parents.get(&node).cloned() {
        path.push(parent.clone());
        node = parent;
    }
    path.into_iter().rev().collect()
}

/// Visit all nodes that are reachable from a start node. The node will be visited
/// is DFS order, starting from the `start` node and following the order returned
/// by the `successors` function.
pub fn dfs_reach<N, FN, IN>(start: N, successors: FN) -> DfsReachable<N, FN>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>, {
    DfsReachable {
        to_see: vec![start],
        visited: HashSet::new(),
        successors,
    }
}

/// Struct returned by [`dfs_reach`].
pub struct DfsReachable<N, FN> {
    to_see: Vec<N>,
    visited: HashSet<N>,
    successors: FN,
}

impl<N, FN> DfsReachable<N, FN>
where
    N: Eq + Hash, {
    /// Return a lower bound on the number of remaining reachable
    /// nodes. Not all nodes are necessarily known is advance, and
    /// new reachable nodes may be discovered while using the iterator.
    pub fn remaining_nodes_low_bound(&self) -> usize {
        self.to_see.iter().collect::<HashSet<_>>().len()
    }
}

impl<N, FN, IN> Iterator for DfsReachable<N, FN>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>, {
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        let n = self.to_see.pop()?;
        if self.visited.contains(&n) {
            return self.next();
        }
        self.visited.insert(n.clone());
        let mut to_insert = Vec::new();
        for s in (self.successors)(&n) {
            if !self.visited.contains(&s) {
                to_insert.push(s.clone());
            }
        }
        self.to_see.extend(to_insert.into_iter().rev());
        Some(n)
    }
}

impl<N, FN, IN> FusedIterator for DfsReachable<N, FN>
where
    N: Eq + Hash + Clone,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = N>, {
}
