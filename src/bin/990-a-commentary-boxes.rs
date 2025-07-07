// Created by Ayush Biswas at 2025/07/07 16:24
// https://codeforces.com/problemset/problem/990/a
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        let b = a.iter().min().unwrap();
        let c = a.iter().max().unwrap();
        (c - b) * (n - 1)
    }
}

// @code end
