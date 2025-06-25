// Created by Ayush Biswas at 2025/05/20 09:23
// https://codeforces.com/problemset/problem/200/B
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: usize,
        a: [usize]
    ) -> f64 {
        a.into_iter().sum::<usize>() as f64 / n as f64
    }
}

// @code end
