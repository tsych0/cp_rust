// Created by Ayush Biswas at 2025/07/06 11:17
// https://codeforces.com/problemset/problem/1632/b
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
    ) -> Words<usize> {
        let m = 2 << ((n-1).ilog2() - 1);
        (0..m).rev().chain(m..n).collect()
    }
}

// @code end
