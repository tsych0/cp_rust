// Created by Ayush Biswas at 2025/06/06 11:34
// https://codeforces.com/problemset/problem/1829/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn (
        _n: usize,
        a: [usize]
    ) -> usize {
        a.into_iter()
             .group_by(|&ai| ai)
             .filter(|g| g[0] == 0)
             .map(|g| g.len())
             .max()
             .unwrap_or(0)
    }
}

// @code end
