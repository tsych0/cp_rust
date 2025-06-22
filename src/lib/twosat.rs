//! A 2-SAT Solver.
use crate::internal_scc;

/// A 2-SAT Solver.
#[derive(Clone, Debug)]
pub struct TwoSat {
    n: usize,
    scc: internal_scc::SccGraph,
    answer: Vec<bool>,
}
impl TwoSat {
    /// Creates a new `TwoSat` of `n` variables and 0 clauses.
    pub fn new(n: usize) -> Self {
        TwoSat {
            n,
            answer: vec![false; n],
            scc: internal_scc::SccGraph::new(2 * n),
        }
    }
    /// Adds a clause $(x_i = f) \lor (x_j = g)$.
    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        assert!(i < self.n && j < self.n);
        self.scc.add_edge(2 * i + !f as usize, 2 * j + g as usize);
        self.scc.add_edge(2 * j + !g as usize, 2 * i + f as usize);
    }
    /// Returns whether there is a truth assignment that satisfies all clauses.
    pub fn satisfiable(&mut self) -> bool {
        let id = self.scc.scc_ids().1;
        for i in 0..self.n {
            if id[2 * i] == id[2 * i + 1] {
                return false;
            }
            self.answer[i] = id[2 * i] < id[2 * i + 1];
        }
        true
    }
    /// Returns a truth assignment that satisfies all clauses **of the last call of [`satisfiable`]**.
    pub fn answer(&self) -> &[bool] {
        &self.answer
    }
}
