pub struct SegmentTree<T, F> {
    n: usize,
    tree: Vec<T>,
    combine: F,
    identity: T,
}

impl<T, F> SegmentTree<T, F>
where
    T: Clone + Copy,
    F: Fn(T, T) -> T,
{
    pub fn new(data: &[T], combine: F, identity: T) -> Self {
        let n = data.len();
        let tree = vec![identity; 4 * n];
        let mut seg_tree = Self {
            n,
            tree,
            combine,
            identity,
        };
        seg_tree.build(data, 1, 0, n - 1);
        seg_tree
    }

    fn build(&mut self, data: &[T], v: usize, tl: usize, tr: usize) {
        if tl == tr {
            self.tree[v] = data[tl];
        } else {
            let tm = (tl + tr) / 2;
            self.build(data, v * 2, tl, tm);
            self.build(data, v * 2 + 1, tm + 1, tr);
            self.tree[v] = (self.combine)(self.tree[v * 2], self.tree[v * 2 + 1]);
        }
    }

    pub fn update(&mut self, pos: usize, val: T) {
        self.update_impl(1, 0, self.n - 1, pos, val);
    }

    fn update_impl(&mut self, v: usize, tl: usize, tr: usize, pos: usize, val: T) {
        if tl == tr {
            self.tree[v] = val;
        } else {
            let tm = (tl + tr) / 2;
            if pos <= tm {
                self.update_impl(v * 2, tl, tm, pos, val);
            } else {
                self.update_impl(v * 2 + 1, tm + 1, tr, pos, val);
            }
            self.tree[v] = (self.combine)(self.tree[v * 2], self.tree[v * 2 + 1]);
        }
    }

    pub fn query(&self, l: usize, r: usize) -> T {
        self.query_impl(1, 0, self.n - 1, l, r)
    }

    fn query_impl(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> T {
        if l > r {
            return self.identity;
        }
        if l == tl && r == tr {
            return self.tree[v];
        }
        let tm = (tl + tr) / 2;
        (self.combine)(
            self.query_impl(v * 2, tl, tm, l, std::cmp::min(r, tm)),
            self.query_impl(v * 2 + 1, tm + 1, tr, std::cmp::max(l, tm + 1), r),
        )
    }
}
