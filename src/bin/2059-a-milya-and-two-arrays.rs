// Created by Ayush Biswas at 2025/05/14 17:56
// https://codeforces.com/problemset/problem/2059/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashSet;

sol! {
    fn solution(
        _n: usize,
        a: [usize],
        b: [usize]
    ) -> bool {
        a.into_iter().collect::<HashSet<_>>().len() + b.into_iter().collect::<HashSet<_>>().len() > 3
    }
}

// @code end
