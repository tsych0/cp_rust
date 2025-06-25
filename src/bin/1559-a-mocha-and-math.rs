// Created by Ayush Biswas at 2025/06/16 15:04
// https://codeforces.com/problemset/problem/1559/A

use cp_lib::*;

use cpio::*;

sol! {
    fn solution(
        _n: usize,
        a: [usize]
    ) -> usize {
        a.into_iter().fold(usize::MAX, |acc, ai| acc & ai)
    }
}

// @code end
