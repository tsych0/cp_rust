// Created by Ayush Biswas at 2025/06/26 23:42
// https://codeforces.com/problemset/problem/1401/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, k]: [usize; 2]
    ) -> usize {
        if k > n {
            k - n
        } else {
            (k^n) % 2
        }
    }
}

// @code end
