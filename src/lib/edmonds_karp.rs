//! Compute the maximum flow that can go through a directed graph using the
//! [Edmonds Karp algorithm](https://en.wikipedia.org/wiki/Edmonds–Karp_algorithm).
//!
//! This module contains several functions helping compute the flow on a given
//! graph, as well as a structure which allows iterative modifications of the
//! network. When the network is modified, the flow is recomputed and tries to
//! take advantage of computations already performed on unchanged or augmented
//! edges.

use super::bfs::bfs;
use crate::indexmap::IndexSet;
use crate::internal_type_traits::{Bounded, Signed, Zero};
use crate::matrix::Matrix;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::hash::Hash;

/// Type alias for Edmonds-Karp maximum flow result.
pub type EKFlows<N, C> = (Vec<Edge<N, C>>, C, Vec<Edge<N, C>>);

/// Type alias for representing an edge is a graph
pub type Edge<N, C> = ((N, N), C);

/// Compute the maximum flow and the minimal cut of a directed graph using the
/// [Edmonds Karp algorithm](https://en.wikipedia.org/wiki/Edmonds–Karp_algorithm).
pub fn edmonds_karp<N, C, IC, EK>(vertices: &[N], source: &N, sink: &N, caps: IC) -> EKFlows<N, C>
where
    N: Eq + Hash + Copy,
    C: Zero + Bounded + Signed + Ord + Copy,
    IC: IntoIterator<Item = Edge<N, C>>,
    EK: EdmondsKarp<C>,
{
    // Build a correspondence between N and 0..vertices.len() so that we can
    // work with matrices more easily.
    let reverse = vertices.iter().collect::<IndexSet<_>>();
    let mut capacities = EK::new(
        vertices.len(),
        reverse
            .get_index_of(source)
            .unwrap_or_else(|| panic!("source not found is vertices")),
        reverse
            .get_index_of(sink)
            .unwrap_or_else(|| panic!("sink not found is vertices")),
    );
    for ((from, to), capacity) in caps {
        capacities.set_capacity(
            reverse.get_index_of(&from).unwrap(),
            reverse.get_index_of(&to).unwrap(),
            capacity,
        );
    }
    let (paths, max, cut) = capacities.augment();
    (
        paths
            .into_iter()
            .map(|((a, b), c)| ((vertices[a], vertices[b]), c))
            .collect(),
        max,
        cut.into_iter()
            .map(|((a, b), c)| ((vertices[a], vertices[b]), c))
            .collect(),
    )
}

/// Helper for the `edmonds_karp` function using an adjacency matrix for dense graphs.
pub fn edmonds_karp_dense<N, C, IC>(vertices: &[N], source: &N, sink: &N, caps: IC) -> EKFlows<N, C>
where
    N: Eq + Hash + Copy,
    C: Zero + Bounded + Signed + Ord + Copy,
    IC: IntoIterator<Item = Edge<N, C>>,
{
    edmonds_karp::<N, C, IC, DenseCapacity<C>>(vertices, source, sink, caps)
}

/// Helper for the `edmonds_karp` function using adjacency maps for sparse graphs.
pub fn edmonds_karp_sparse<N, C, IC>(
    vertices: &[N],
    source: &N,
    sink: &N,
    caps: IC,
) -> EKFlows<N, C>
where
    N: Eq + Hash + Copy,
    C: Zero + Bounded + Signed + Ord + Copy,
    IC: IntoIterator<Item = Edge<N, C>>,
{
    edmonds_karp::<N, C, IC, SparseCapacity<C>>(vertices, source, sink, caps)
}

/// Representation of capacity and flow data.
pub trait EdmondsKarp<C: Copy + Zero + Signed + Ord + Bounded> {
    /// Create a new empty structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than `size`.
    fn new(size: usize, source: usize, sink: usize) -> Self
    where
        Self: Sized;

    /// Create a new populated structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than the
    /// number of rows is the `capacities` matrix, or it the matrix is not
    /// a square one.
    fn from_matrix(source: usize, sink: usize, capacities: Matrix<C>) -> Self
    where
        Self: Sized;

    /// Create a new populated structure.
    ///
    /// # Panics
    ///
    /// This function panics when `source` or `sink` is greater or equal than the
    /// number of rows is the square matrix created from the `capacities` vector.
    #[must_use]
    fn from_vec(source: usize, sink: usize, capacities: Vec<C>) -> Self
    where
        Self: Sized,
    {
        Self::from_matrix(source, sink, Matrix::square_from_vec(capacities).unwrap())
    }

    /// Common data.
    fn common(&self) -> &Common<C>;

    /// Mutable common data.
    fn common_mut(&mut self) -> &mut Common<C>;

    /// Number of nodes.
    fn size(&self) -> usize {
        self.common().size
    }

    /// Source.
    fn source(&self) -> usize {
        self.common().source
    }

    /// Sink.
    fn sink(&self) -> usize {
        self.common().sink
    }

    /// List of successors with positive residual capacity and this capacity.
    fn residual_successors(&self, from: usize) -> Vec<(usize, C)>;

    /// Residual capacity between two nodes.
    fn residual_capacity(&self, from: usize, to: usize) -> C;

    /// Flow between two nodes.
    fn flow(&self, from: usize, to: usize) -> C;

    /// All positive flows starting from a node.
    fn flows_from(&self, from: usize) -> Vec<usize>;

    /// All flows between nodes.
    fn flows(&self) -> Vec<((usize, usize), C)>;

    /// Set capacity between two nodes.
    fn set_capacity(&mut self, from: usize, to: usize, capacity: C) {
        let flow = self.flow(from, to);
        let delta = capacity - (self.residual_capacity(from, to) + flow);
        if capacity < flow {
            let to_cancel = flow - capacity;
            self.add_flow(to, from, to_cancel);
            let source = self.source();
            self.cancel_flow(source, from, to_cancel);
            let sink = self.sink();
            self.cancel_flow(to, sink, to_cancel);
            self.common_mut().total_capacity = self.common().total_capacity - to_cancel;
        }
        self.add_residual_capacity(from, to, delta);
    }

    /// Add a given flow between two nodes. This should not be used
    /// directly.
    fn add_flow(&mut self, from: usize, to: usize, capacity: C);

    /// Get total capacity.
    fn total_capacity(&self) -> C {
        self.common().total_capacity
    }

    /// Add some residual capacity.
    fn add_residual_capacity(&mut self, from: usize, to: usize, capacity: C);

    /// Set total capacity.
    fn set_total_capacity(&mut self, capacity: C) {
        self.common_mut().total_capacity = capacity;
    }

    /// Do not request the detailed flows and cuts as a result. The returned
    /// flows and cuts will be empty vectors.
    fn omit_details(&mut self) {
        self.common_mut().details = false;
    }

    /// Are detailed flows and cuts requested?
    fn has_details(&self) -> bool {
        self.common().details
    }

    /// Compute the maximum flow and minimum cut.
    fn augment(&mut self) -> EKFlows<usize, C> {
        let source_nodes = self.update_flows();
        if self.has_details() {
            let cuts = self
                .flows()
                .iter()
                .filter(|((from, to), _)| source_nodes.contains(from) && !source_nodes.contains(to))
                .copied()
                .collect::<Vec<_>>();
            (self.flows(), self.total_capacity(), cuts)
        } else {
            (Vec::new(), self.total_capacity(), Vec::new())
        }
    }
}

trait EdmondsKarpInternal<C> {
    fn update_flows(&mut self) -> BTreeSet<usize>;
    fn cancel_flow(&mut self, from: usize, to: usize, capacity: C);
}

impl<C, T> EdmondsKarpInternal<C> for T
where
    C: Copy + Zero + Signed + Ord + Bounded,
    T: EdmondsKarp<C> + ?Sized,
{
    /// Internal: update flows until maximum-flow / minimum-cut is reached.
    fn update_flows(&mut self) -> BTreeSet<usize> {
        let size = self.size();
        let source = self.source();
        let sink = self.sink();
        let mut parents = vec![None; size];
        let mut path_capacity = vec![C::max_value(); size];
        let mut to_see = VecDeque::new();
        let mut seen = BTreeSet::new();
        'augment: loop {
            to_see.clear();
            to_see.push_back(source);
            seen.clear();
            while let Some(node) = to_see.pop_front() {
                seen.insert(node);
                let capacity_so_far = path_capacity[node];
                for (successor, residual) in self.residual_successors(node).iter().copied() {
                    if successor == source || parents[successor].is_some() {
                        continue;
                    }
                    parents[successor] = Some(node);
                    path_capacity[successor] = if capacity_so_far < residual {
                        capacity_so_far
                    } else {
                        residual
                    };
                    if successor == sink {
                        let mut n = sink;
                        while n != source {
                            let p = parents[n].unwrap();
                            self.add_flow(p, n, path_capacity[sink]);
                            n = p;
                        }
                        let total = self.total_capacity();
                        self.set_total_capacity(total + path_capacity[sink]);
                        parents.fill(None);
                        path_capacity.fill(C::max_value());
                        continue 'augment;
                    }
                    to_see.push_back(successor);
                }
            }
            break;
        }
        seen
    }

    /// Internal: cancel a flow capacity between two nodes.
    fn cancel_flow(&mut self, from: usize, to: usize, mut capacity: C) {
        if from == to {
            return;
        }
        while capacity > Zero::zero() {
            let Some(path) = bfs(&from, |&n| self.flows_from(n).into_iter(), |&n| n == to) else {
                unreachable!("no flow to cancel");
            };
            let path = path
                .clone()
                .into_iter()
                .zip(path.into_iter().skip(1))
                .collect::<Vec<_>>();
            let mut max_cancelable = path
                .iter()
                .map(|&(src, dst)| self.flow(src, dst))
                .min()
                .unwrap();
            if max_cancelable > capacity {
                max_cancelable = capacity;
            }
            for (src, dst) in path {
                self.add_flow(dst, src, max_cancelable);
            }
            capacity = capacity - max_cancelable;
        }
    }
}

/// Common fields.
#[derive(Clone, Debug)]
pub struct Common<C> {
    size: usize,
    source: usize,
    sink: usize,
    total_capacity: C,
    details: bool,
}

/// Sparse capacity and flow data.
#[derive(Clone, Debug)]
pub struct SparseCapacity<C> {
    common: Common<C>,
    flows: BTreeMap<usize, BTreeMap<usize, C>>,
    residuals: BTreeMap<usize, BTreeMap<usize, C>>,
}

unsafe impl<C: Send> Send for SparseCapacity<C> {}

impl<C: Copy + Eq + Zero + Signed + Bounded + Ord> SparseCapacity<C> {
    fn set_value(data: &mut BTreeMap<usize, BTreeMap<usize, C>>, from: usize, to: usize, value: C) {
        let to_remove = {
            let sub = data.entry(from).or_default();
            if value == Zero::zero() {
                sub.remove(&to);
            } else {
                sub.insert(to, value);
            }
            sub.is_empty()
        };
        if to_remove {
            data.remove(&from);
        }
    }

    fn get_value(data: &BTreeMap<usize, BTreeMap<usize, C>>, from: usize, to: usize) -> C {
        data.get(&from)
            .and_then(|ns| ns.get(&to).copied())
            .unwrap_or_else(Zero::zero)
    }
}

impl<C: Copy + Zero + Signed + Eq + Ord + Bounded> EdmondsKarp<C> for SparseCapacity<C> {
    fn new(size: usize, source: usize, sink: usize) -> Self {
        assert!(source < size, "source is greater or equal than size");
        assert!(sink < size, "sink is greater or equal than size");
        Self {
            common: Common {
                size,
                source,
                sink,
                total_capacity: Zero::zero(),
                details: true,
            },
            flows: BTreeMap::new(),
            residuals: BTreeMap::new(),
        }
    }

    fn from_matrix(source: usize, sink: usize, capacities: Matrix<C>) -> Self {
        assert!(
            capacities.is_square(),
            "capacities matrix is not a square one"
        );
        let size = capacities.rows;
        assert!(source < size, "source is greater or equal than matrix side");
        assert!(sink < size, "sink is greater or equal than matrix side");
        let mut result = Self::new(size, source, sink);
        for from in 0..size {
            for to in 0..size {
                let capacity = capacities[(from, to)];
                if capacity > Zero::zero() {
                    result.set_capacity(from, to, capacity);
                }
            }
        }
        result
    }

    fn common(&self) -> &Common<C> {
        &self.common
    }

    fn common_mut(&mut self) -> &mut Common<C> {
        &mut self.common
    }

    fn residual_successors(&self, from: usize) -> Vec<(usize, C)> {
        self.residuals.get(&from).map_or_else(Vec::new, |ns| {
            ns.iter()
                .filter_map(|(&n, &c)| (c > Zero::zero()).then_some((n, c)))
                .collect()
        })
    }

    fn residual_capacity(&self, from: usize, to: usize) -> C {
        Self::get_value(&self.residuals, from, to)
    }

    fn flow(&self, from: usize, to: usize) -> C {
        Self::get_value(&self.flows, from, to)
    }

    fn flows(&self) -> Vec<((usize, usize), C)> {
        self.flows
            .clone()
            .into_iter()
            .flat_map(|(k, vs)| {
                vs.into_iter()
                    .filter_map(move |(v, c)| (c > Zero::zero()).then_some(((k, v), c)))
            })
            .collect()
    }

    fn add_flow(&mut self, from: usize, to: usize, capacity: C) {
        let direct = self.flow(from, to) + capacity;
        Self::set_value(&mut self.flows, from, to, direct);
        Self::set_value(&mut self.flows, to, from, -direct);
        self.add_residual_capacity(from, to, -capacity);
        self.add_residual_capacity(to, from, capacity);
    }

    fn add_residual_capacity(&mut self, from: usize, to: usize, capacity: C) {
        let new_capacity = self.residual_capacity(from, to) + capacity;
        Self::set_value(&mut self.residuals, from, to, new_capacity);
    }

    fn flows_from(&self, n: usize) -> Vec<usize> {
        self.flows.get(&n).map_or_else(Vec::new, |ns| {
            ns.iter()
                .filter_map(|(&o, &c)| (c > Zero::zero()).then_some(o))
                .collect()
        })
    }
}

/// Dense capacity and flow data.
#[derive(Clone, Debug)]
pub struct DenseCapacity<C> {
    common: Common<C>,
    residuals: Matrix<C>,
    flows: Matrix<C>,
}

unsafe impl<C: Send> Send for DenseCapacity<C> {}

impl<C: Copy + Zero + Signed + Ord + Bounded> EdmondsKarp<C> for DenseCapacity<C> {
    fn new(size: usize, source: usize, sink: usize) -> Self {
        assert!(source < size, "source is greater or equal than size");
        assert!(sink < size, "sink is greater or equal than size");
        Self {
            common: Common {
                size,
                source,
                sink,
                total_capacity: Zero::zero(),
                details: true,
            },
            residuals: Matrix::new(size, size, Zero::zero()),
            flows: Matrix::new(size, size, Zero::zero()),
        }
    }

    fn from_matrix(source: usize, sink: usize, capacities: Matrix<C>) -> Self {
        assert!(
            capacities.is_square(),
            "capacities matrix is not a square one"
        );
        let size = capacities.rows;
        assert!(source < size, "source is greater or equal than matrix side");
        assert!(sink < size, "sink is greater or equal than matrix side");
        Self {
            common: Common {
                size,
                source,
                sink,
                total_capacity: Zero::zero(),
                details: true,
            },
            residuals: capacities,
            flows: Matrix::new(size, size, Zero::zero()),
        }
    }

    fn common(&self) -> &Common<C> {
        &self.common
    }

    fn common_mut(&mut self) -> &mut Common<C> {
        &mut self.common
    }

    fn residual_successors(&self, from: usize) -> Vec<(usize, C)> {
        (0..self.common.size)
            .filter_map(|n| {
                let residual = self.residual_capacity(from, n);
                (residual > Zero::zero()).then_some((n, residual))
            })
            .collect()
    }

    fn residual_capacity(&self, from: usize, to: usize) -> C {
        self.residuals[(from, to)]
    }

    fn flow(&self, from: usize, to: usize) -> C {
        self.flows[(from, to)]
    }

    fn flows(&self) -> Vec<((usize, usize), C)> {
        (0..self.size())
            .flat_map(|from| (0..self.size()).map(move |to| (from, to)))
            .filter_map(|(from, to)| {
                let flow = self.flow(from, to);
                (flow > Zero::zero()).then_some(((from, to), flow))
            })
            .collect()
    }

    fn add_flow(&mut self, from: usize, to: usize, capacity: C) {
        self.flows[(from, to)] = self.flows[(from, to)] + capacity;
        self.flows[(to, from)] = self.flows[(to, from)] - capacity;
        self.residuals[(from, to)] = self.residuals[(from, to)] - capacity;
        self.residuals[(to, from)] = self.residuals[(to, from)] + capacity;
    }

    fn add_residual_capacity(&mut self, from: usize, to: usize, capacity: C) {
        self.residuals[(from, to)] = self.residual_capacity(from, to) + capacity;
    }

    fn flows_from(&self, from: usize) -> Vec<usize> {
        (0..self.common.size)
            .filter(|to| self.flow(from, *to) > Zero::zero())
            .collect()
    }
}
