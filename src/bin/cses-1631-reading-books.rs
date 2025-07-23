// Created by Ayush Biswas at 2025/07/18 15:18
// https://cses.fi/problemset/task/1631
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        (mut a): [usize]
    ) -> usize {
        a.sort();
        a[0..n-1].into_iter().sum::<usize>().max(a[n-1]) + a[n-1]
    }
}

// @code end
