// Created by Ayush Biswas at 2025/07/06 11:11
// https://codeforces.com/problemset/problem/1197/B
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> bool {
        let max_idx = a.iter().zip(0..n).max().unwrap().1;
        Itertools::is_sorted(a[0..max_idx].into_iter()) &&
            Itertools::is_sorted(a[max_idx..n].into_iter().rev())
    }
}

// @code end
