// Created by Ayush Biswas at 2025/05/17 14:18
// https://codeforces.com/problemset/problem/2013/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        [x, y]: [usize; 2]
    ) -> usize {
        n.div_ceil(x.min(y))
    }
}

// @code end
