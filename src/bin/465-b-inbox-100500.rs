// Created by Ayush Biswas at 2025/07/04 12:24
// https://codeforces.com/problemset/problem/465/B
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        a
            .into_iter()
            .group_by(|&ai| ai)
            .filter(|g| g[0] == 1)
            .map(|g| g.len() + 1)
            .sum::<usize>()
            .saturating_sub(1)
    }
}

// @code end
