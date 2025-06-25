pub struct BinaryLifting {
    adj: Vec<Vec<usize>>,
    up: Vec<Vec<Option<usize>>>,
    depth: Vec<usize>,
    log: usize,
    _n: usize,
}

impl BinaryLifting {
    pub fn new(n: usize) -> Self {
        let log = (n as f64).log2().ceil() as usize + 1;
        Self {
            adj: vec![Vec::new(); n],
            up: vec![vec![None; log]; n],
            depth: vec![0; n],
            log,
            _n: n,
        }
    }

    pub fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }

    fn dfs(&mut self, v: usize, parent: Option<usize>) {
        self.up[v][0] = parent;

        // Fill the binary lifting table
        for j in 1..self.log {
            if let Some(prev) = self.up[v][j - 1] {
                self.up[v][j] = self.up[prev][j - 1];
            }
        }

        // Recurse to children
        for &u in &self.adj[v].clone() {
            if Some(u) != parent {
                self.depth[u] = self.depth[v] + 1;
                self.dfs(u, Some(v));
            }
        }
    }

    pub fn preprocess(&mut self, root: usize) {
        self.depth[root] = 0;
        self.dfs(root, None);
    }

    pub fn lift(&self, mut node: usize, k: usize) -> Option<usize> {
        for j in 0..self.log {
            if k & (1 << j) != 0 {
                if let Some(next_node) = self.up[node][j] {
                    node = next_node;
                } else {
                    return None;
                }
            }
        }
        Some(node)
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> Option<usize> {
        // Ensure u is deeper than v
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }

        // Lift u to the same depth as v
        let depth_diff = self.depth[u] - self.depth[v];
        u = self.lift(u, depth_diff)?;

        // If they're the same node now, that's the LCA
        if u == v {
            return Some(u);
        }

        // Binary search for LCA
        for j in (0..self.log).rev() {
            if let (Some(u_ancestor), Some(v_ancestor)) = (self.up[u][j], self.up[v][j]) {
                if u_ancestor != v_ancestor {
                    u = u_ancestor;
                    v = v_ancestor;
                }
            }
        }

        self.up[u][0]
    }

    pub fn distance(&self, u: usize, v: usize) -> Option<usize> {
        let lca_node = self.lca(u, v)?;
        Some(self.depth[u] + self.depth[v] - 2 * self.depth[lca_node])
    }

    pub fn is_ancestor(&self, ancestor: usize, node: usize) -> bool {
        if self.depth[ancestor] > self.depth[node] {
            return false;
        }

        let depth_diff = self.depth[node] - self.depth[ancestor];
        if let Some(lifted) = self.lift(node, depth_diff) {
            lifted == ancestor
        } else {
            false
        }
    }

    pub fn kth_ancestor(&self, node: usize, k: usize) -> Option<usize> {
        if k > self.depth[node] {
            None
        } else {
            self.lift(node, k)
        }
    }

    pub fn path_nodes(&self, u: usize, v: usize) -> Option<Vec<usize>> {
        let lca_node = self.lca(u, v)?;
        let mut path = Vec::new();

        // Path from u to LCA
        let mut current = u;
        while current != lca_node {
            path.push(current);
            current = self.up[current][0]?;
        }

        // Add LCA
        path.push(lca_node);

        // Path from LCA to v (in reverse)
        let mut v_path = Vec::new();
        current = v;
        while current != lca_node {
            v_path.push(current);
            current = self.up[current][0]?;
        }

        // Add v_path is reverse order
        path.extend(v_path.into_iter().rev());

        Some(path)
    }

    pub fn kth_node_on_path(&self, u: usize, v: usize, k: usize) -> Option<usize> {
        let lca_node = self.lca(u, v)?;
        let dist_u_lca = self.depth[u] - self.depth[lca_node];
        let total_distance = self.distance(u, v)?;

        if k > total_distance {
            return None;
        }

        if k <= dist_u_lca {
            // kth node is on path from u to LCA
            self.lift(u, k)
        } else {
            // kth node is on path from LCA to v
            let steps_from_lca = k - dist_u_lca;
            let dist_v_lca = self.depth[v] - self.depth[lca_node];
            self.lift(v, dist_v_lca - steps_from_lca)
        }
    }
}
