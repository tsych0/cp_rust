// Created by Ayush Biswas at 2025/06/12 15:11
// https://codeforces.com/problemset/problem/337/A

use cp_lib::{itertools::Itertools, *};

// @code begin
use cpio::*;
use std::usize;

sol! {
    fn solution(
        [n, m]: [usize; 2],
        f: [usize]
    ) -> usize {
        let f: Vec<_> = f.into_iter().sorted().collect();
        let mut res = usize::MAX;
        for i in 0..m - n + 1 {
            res = res.min(f[i + n - 1] - f[i])
        }
        res
    }
}

// @code end
