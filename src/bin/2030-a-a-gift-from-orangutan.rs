// Created by Ayush Biswas at 2025/05/16 17:21
// https://codeforces.com/problemset/problem/2030/A
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
