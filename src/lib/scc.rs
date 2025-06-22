use crate::internal_scc;

/// An `SccGraph` is a directed graph that calculates strongly connected components (SCC) in $O(|V| + |E|)$.
#[derive(Clone, Debug)]
pub struct SccGraph {
    internal: internal_scc::SccGraph,
}

impl SccGraph {
    /// Creates a new `SccGraph` with `n` vertices and `0` edges.
    pub fn new(n: usize) -> Self {
        SccGraph {
            internal: internal_scc::SccGraph::new(n),
        }
    }

    /// Adds a directed edge from the vertex `from` to the vertex `to`.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        let n = self.internal.num_vertices();
        assert!(from < n);
        assert!(to < n);
        self.internal.add_edge(from, to);
    }

    /// Calculates the strongly connected components (SCC) of directed graphs in $O(|V| + |E|)$.
    pub fn scc(&self) -> Vec<Vec<usize>> {
        self.internal.scc()
    }
}
