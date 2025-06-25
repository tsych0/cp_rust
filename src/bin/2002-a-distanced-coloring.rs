// Created by Ayush Biswas at 2025/05/18 20:16
// https://codeforces.com/problemset/problem/2002/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [n, m, k]: [usize; 3]
    ) -> usize {
        n.min(k) * m.min(k)
    }
}

// @code end
