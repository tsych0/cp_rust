// Created by Ayush Biswas at 2025/06/12 23:42
// https://codeforces.com/problemset/problem/1879/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::min;

sol_n! {
    fn solution(
        n: usize,
        a: [usize],
        b: [usize]
    ) -> usize {
        let (a_sum, a_min) = (a.iter().sum::<usize>(), a.iter().min().unwrap());
        let (b_sum, b_min) = (b.iter().sum::<usize>(), b.iter().min().unwrap());
        min(a_sum + b_min * n, b_sum + a_min * n)
    }
}

// @code end
