// Created by Ayush Biswas at 2025/07/02 11:08
// https://codeforces.com/problemset/problem/1/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, m, a]: [usize; 3]
    ) -> usize {
        n.div_ceil(a) * m.div_ceil(a)
    }
}

// @code end
