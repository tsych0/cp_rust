// Created by Ayush Biswas at 2025/06/27 10:11
// https://codeforces.com/problemset/problem/1335/B
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, a, b]: [usize; 3]
    ) -> String {
        ('a'..='z').take(b).cycle().take(a).cycle().take(n).collect()
    }
}

// @code end
