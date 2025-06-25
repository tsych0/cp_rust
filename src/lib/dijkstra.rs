//! Compute a shortest path using the [Dijkstra search
//! algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).

use crate::indexmap::{
    Entry::{Occupied, Vacant},
    IndexMap,
};
use crate::internal_type_traits::Zero;
use crate::pathfinding::reverse_path;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;
use std::{cmp::Ordering, collections::HashSet};

/// Compute a shortest path using the [Dijkstra search
/// algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
pub fn dijkstra<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    dijkstra_internal(start, &mut successors, &mut success)
}

pub(crate) fn dijkstra_internal<N, C, FN, IN, FS>(
    start: &N,
    successors: &mut FN,
    success: &mut FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let (parents, reached) = run_dijkstra(start, successors, success);
    reached.map(|target| {
        (
            reverse_path(&parents, |&(p, _)| p, target),
            parents.get_index(target).unwrap().1 .1,
        )
    })
}

/// Determine all reachable nodes from a starting point as well as the
/// minimum cost to reach them and a possible optimal parent node
/// using the [Dijkstra search
/// algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
pub fn dijkstra_all<N, C, FN, IN>(start: &N, successors: FN) -> HashMap<N, (N, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    dijkstra_partial(start, successors, |_| false).0
}

/// Determine some reachable nodes from a starting point as well as the minimum cost to
/// reach them and a possible optimal parent node
/// using the [Dijkstra search algorithm](https://en.wikipedia.org/wiki/Dijkstra's_algorithm).
#[expect(clippy::missing_panics_doc)]
pub fn dijkstra_partial<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut stop: FS,
) -> (HashMap<N, (N, C)>, Option<N>)
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let (parents, reached) = run_dijkstra(start, &mut successors, &mut stop);
    (
        parents
            .iter()
            .skip(1)
            .map(|(n, (p, c))| (n.clone(), (parents.get_index(*p).unwrap().0.clone(), *c))) // unwrap() cannot fail
            .collect(),
        reached.map(|i| parents.get_index(i).unwrap().0.clone()),
    )
}

fn run_dijkstra<N, C, FN, IN, FS>(
    start: &N,
    successors: &mut FN,
    stop: &mut FS,
) -> (IndexMap<N, (usize, C)>, Option<usize>)
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestHolder {
        cost: Zero::zero(),
        index: 0,
    });
    let mut parents: IndexMap<N, (usize, C)> = IndexMap::new();
    parents.insert(start.clone(), (usize::MAX, Zero::zero()));
    let mut target_reached = None;
    while let Some(SmallestHolder { cost, index }) = to_see.pop() {
        let successors = {
            let (node, &(_, c)) = parents.get_index(index).unwrap();
            if stop(node) {
                target_reached = Some(index);
                break;
            }
            // We may have inserted a node several time into the binary heap if we found
            // a better way to access it. Ensure that we are currently dealing with the
            // best path and discard the others.
            if cost > c {
                continue;
            }
            successors(node)
        };
        for (successor, move_cost) in successors {
            let new_cost = cost + move_cost;
            let n;
            match parents.entry(successor) {
                Vacant(e) => {
                    n = e.index();
                    e.insert((index, new_cost));
                }
                Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        n = e.index();
                        e.insert((index, new_cost));
                    } else {
                        continue;
                    }
                }
            }

            to_see.push(SmallestHolder {
                cost: new_cost,
                index: n,
            });
        }
    }
    (parents, target_reached)
}

/// Build a path leading to a target according to a parents map, which must
/// contain no loop. This function can be used after [`dijkstra_all`] or
/// [`dijkstra_partial`] to build a path from a starting point to a reachable target.
#[expect(clippy::implicit_hasher)]
pub fn build_path<N, C>(target: &N, parents: &HashMap<N, (N, C)>) -> Vec<N>
where
    N: Eq + Hash + Clone,
{
    let mut rev = vec![target.clone()];
    let mut next = target.clone();
    while let Some((parent, _)) = parents.get(&next) {
        rev.push(parent.clone());
        next = parent.clone();
    }
    rev.reverse();
    rev
}

struct SmallestHolder<K> {
    cost: K,
    index: usize,
}

impl<K: PartialEq> PartialEq for SmallestHolder<K> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<K: PartialEq> Eq for SmallestHolder<K> {}

impl<K: Ord> PartialOrd for SmallestHolder<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord> Ord for SmallestHolder<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

/// Struct returned by [`dijkstra_reach`].
pub struct DijkstraReachable<N, C, FN> {
    to_see: BinaryHeap<SmallestHolder<C>>,
    seen: HashSet<usize>,
    parents: IndexMap<N, (usize, C)>,
    total_costs: HashMap<N, C>,
    successors: FN,
}

/// Information about a node reached by [`dijkstra_reach`].
#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct DijkstraReachableItem<N, C> {
    /// The node that was reached by [`dijkstra_reach`].
    pub node: N,
    /// The previous node that the current node came from.
    /// If the node is the first node, there will be no parent.
    pub parent: Option<N>,
    /// The total cost from the starting node.
    pub total_cost: C,
}

impl<N, C, FN, IN> Iterator for DijkstraReachable<N, C, FN>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy + Hash,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    type Item = DijkstraReachableItem<N, C>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(SmallestHolder { cost, index }) = self.to_see.pop() {
            if !self.seen.insert(index) {
                continue;
            }
            let item;
            let successors = {
                let (node, (parent_index, _)) = self.parents.get_index(index).unwrap();
                let total_cost = self.total_costs[node];
                item = Some(DijkstraReachableItem {
                    node: node.clone(),
                    parent: self.parents.get_index(*parent_index).map(|x| x.0.clone()),
                    total_cost,
                });
                (self.successors)(node)
            };
            for (successor, move_cost) in successors {
                let new_cost = cost + move_cost;
                let n;
                match self.parents.entry(successor.clone()) {
                    Vacant(e) => {
                        n = e.index();
                        e.insert((index, new_cost));
                        self.total_costs.insert(successor.clone(), new_cost);
                    }
                    Occupied(mut e) => {
                        if e.get().1 > new_cost {
                            n = e.index();
                            e.insert((index, new_cost));
                            self.total_costs.insert(successor.clone(), new_cost);
                        } else {
                            continue;
                        }
                    }
                }

                self.to_see.push(SmallestHolder {
                    cost: new_cost,
                    index: n,
                });
            }
            return item;
        }

        None
    }
}

/// Visit all nodes that are reachable from a start node. The node
/// will be visited is order of cost, with the closest nodes first.
pub fn dijkstra_reach<N, C, FN, IN>(start: &N, successors: FN) -> DijkstraReachable<N, C, FN>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    let mut to_see = BinaryHeap::new();
    to_see.push(SmallestHolder {
        cost: Zero::zero(),
        index: 0,
    });

    let mut parents: IndexMap<N, (usize, C)> = IndexMap::new();
    parents.insert(start.clone(), (usize::MAX, Zero::zero()));

    let mut total_costs = HashMap::default();
    total_costs.insert(start.clone(), Zero::zero());

    let seen = HashSet::default();

    DijkstraReachable {
        to_see,
        seen,
        parents,
        total_costs,
        successors,
    }
}
