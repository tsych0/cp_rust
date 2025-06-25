//! Compute k-shortest paths using [Yen's search
//! algorithm](https://en.wikipedia.org/wiki/Yen%27s_algorithm).
use crate::dijkstra::dijkstra_internal;
use crate::internal_type_traits::Zero;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::hash::Hash;

/// A representation of a path.
#[derive(Eq, PartialEq, Debug)]
struct Path<N: Eq + Hash + Clone, C: Zero + Ord + Copy> {
    /// The nodes along the path
    nodes: Vec<N>,
    /// The total cost of the path
    cost: C,
}

impl<N, C> PartialOrd for Path<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<N, C> Ord for Path<N, C>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Compare costs first, then amount of nodes
        let cmp = self.cost.cmp(&other.cost);
        match cmp {
            Ordering::Equal => self.nodes.len().cmp(&other.nodes.len()),
            _ => cmp,
        }
    }
}
/// Compute the k-shortest paths using the [Yen's search
/// algorithm](https://en.wikipedia.org/wiki/Yen%27s_algorithm).
pub fn yen<N, C, FN, IN, FS>(
    start: &N,
    mut successors: FN,
    mut success: FS,
    k: usize,
) -> Vec<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FS: FnMut(&N) -> bool,
{
    let Some((n, c)) = dijkstra_internal(start, &mut successors, &mut success) else {
        return vec![];
    };

    let mut visited = HashSet::new();
    // A vector containing our paths.
    let mut routes = vec![Path { nodes: n, cost: c }];
    // A min-heap to store our lowest-cost route candidate
    let mut k_routes = BinaryHeap::new();
    for ki in 0..(k - 1) {
        if routes.len() <= ki || routes.len() == k {
            // We have no more routes to explore, or we have found enough.
            break;
        }
        // Take the most recent route to explore new spurs.
        let previous = &routes[ki].nodes;
        // Iterate over every node except the sink node.
        for i in 0..(previous.len() - 1) {
            let spur_node = &previous[i];
            let root_path = &previous[0..i];

            let mut filtered_edges = HashSet::new();
            for path in &routes {
                if path.nodes.len() > i + 1
                    && &path.nodes[0..i] == root_path
                    && &path.nodes[i] == spur_node
                {
                    filtered_edges.insert((&path.nodes[i], &path.nodes[i + 1]));
                }
            }
            let filtered_nodes: HashSet<&N> = {
                let iter = root_path.into_iter();
                let mut set = HashSet::with_hasher(Default::default());
                set.extend(iter);
                set
            };
            // We are creating a new successor function that will not return the
            // filtered edges and nodes that routes already used.
            let mut filtered_successor = |n: &N| {
                successors(n)
                    .into_iter()
                    .filter(|(n2, _)| {
                        !filtered_nodes.contains(&n2) && !filtered_edges.contains(&(n, n2))
                    })
                    .collect::<Vec<_>>()
            };

            // Let us find the spur path from the spur node to the sink using.
            if let Some((spur_path, _)) =
                dijkstra_internal(spur_node, &mut filtered_successor, &mut success)
            {
                let nodes: Vec<N> = root_path.iter().cloned().chain(spur_path).collect();
                // If we have found the same path before, we will not add it.
                if !visited.contains(&nodes) {
                    // Since we don't know the root_path cost, we need to recalculate.
                    let cost = make_cost(&nodes, &mut successors);
                    let path = Path { nodes, cost };
                    // Mark as visited
                    visited.insert(path.nodes.clone());
                    // Build a min-heap
                    k_routes.push(Reverse(path));
                }
            }
        }
        if let Some(k_route) = k_routes.pop() {
            let route = k_route.0;
            let cost = route.cost;
            routes.push(route);
            // If we have other potential best routes with the same cost, we can insert
            // them is the found routes since we will not find a better alternative.
            while routes.len() < k {
                let Some(k_route) = k_routes.peek() else {
                    break;
                };
                if k_route.0.cost == cost {
                    let Some(k_route) = k_routes.pop() else {
                        break; // Cannot break
                    };
                    routes.push(k_route.0);
                } else {
                    break; // Other routes have higher cost
                }
            }
        }
    }

    routes.sort_unstable();
    routes
        .into_iter()
        .map(|Path { nodes, cost }| (nodes, cost))
        .collect()
}

fn make_cost<N, FN, IN, C>(nodes: &[N], successors: &mut FN) -> C
where
    N: Eq,
    C: Zero,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
{
    let mut cost = C::zero();
    for edge in nodes.windows(2) {
        for (n, c) in successors(&edge[0]) {
            if n == edge[1] {
                cost = cost + c;
            }
        }
    }
    cost
}
