use std::cmp::{max, min};

pub struct LazySegmentTree<T, U, F1, F2, F3>
where
    T: Clone + Copy,
    U: Clone + Copy + PartialEq,
    F1: Fn(T, T) -> T,
    F2: Fn(T, U, usize) -> T,
    F3: Fn(U, U) -> U,
{
    n: usize,
    tree: Vec<T>,
    lazy: Vec<U>,
    combine: F1,
    apply: F2,
    merge_update: F3,
    identity: T,
    no_update: U,
}

impl<T, U, F1, F2, F3> LazySegmentTree<T, U, F1, F2, F3>
where
    T: Clone + Copy,
    U: Clone + Copy + PartialEq,
    F1: Fn(T, T) -> T,
    F2: Fn(T, U, usize) -> T,
    F3: Fn(U, U) -> U,
{
    pub fn new(
        data: &[T],
        combine: F1,
        apply: F2,
        merge_update: F3,
        identity: T,
        no_update: U,
    ) -> Self {
        let n = data.len();
        let mut seg_tree = Self {
            n,
            tree: vec![identity; 4 * n],
            lazy: vec![no_update; 4 * n],
            combine,
            apply,
            merge_update,
            identity,
            no_update,
        };
        seg_tree.build(data, 1, 0, n - 1);
        seg_tree
    }

    fn push(&mut self, v: usize, l: usize, r: usize) {
        if self.lazy[v] != self.no_update {
            self.tree[v] = (self.apply)(self.tree[v], self.lazy[v], r - l + 1);
            if l != r {
                self.lazy[v * 2] = (self.merge_update)(self.lazy[v * 2], self.lazy[v]);
                self.lazy[v * 2 + 1] = (self.merge_update)(self.lazy[v * 2 + 1], self.lazy[v]);
            }
            self.lazy[v] = self.no_update;
        }
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

    fn update_impl(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize, val: U) {
        self.push(v, tl, tr);
        if l > r {
            return;
        }
        if l == tl && r == tr {
            self.lazy[v] = (self.merge_update)(self.lazy[v], val);
            self.push(v, tl, tr);
        } else {
            let tm = (tl + tr) / 2;
            self.update_impl(v * 2, tl, tm, l, min(r, tm), val);
            self.update_impl(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r, val);
            self.push(v * 2, tl, tm);
            self.push(v * 2 + 1, tm + 1, tr);
            self.tree[v] = (self.combine)(self.tree[v * 2], self.tree[v * 2 + 1]);
        }
    }

    fn query_impl(&mut self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> T {
        self.push(v, tl, tr);
        if l > r {
            return self.identity;
        }
        if l == tl && r == tr {
            return self.tree[v];
        }
        let tm = (tl + tr) / 2;
        let left = self.query_impl(v * 2, tl, tm, l, min(r, tm));
        let right = self.query_impl(v * 2 + 1, tm + 1, tr, max(l, tm + 1), r);
        (self.combine)(left, right)
    }

    pub fn update(&mut self, l: usize, r: usize, val: U) {
        self.update_impl(1, 0, self.n - 1, l, r, val);
    }

    pub fn query(&mut self, l: usize, r: usize) -> T {
        self.query_impl(1, 0, self.n - 1, l, r)
    }

    pub fn point_query(&mut self, pos: usize) -> T {
        self.query(pos, pos)
    }

    pub fn point_update(&mut self, pos: usize, val: U) {
        self.update(pos, pos, val);
    }
}
