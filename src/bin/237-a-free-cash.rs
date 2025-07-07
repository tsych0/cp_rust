// Created by Ayush Biswas at 2025/07/04 13:02
// https://codeforces.com/contest/237/problem/A
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        n: usize,
        arrivals: [[usize; 2]; n]
    ) -> usize {
        arrivals
            .into_iter()
            .sorted()
            .group_by(|&ai| ai)
            .map(|g| g.len())
            .max()
            .unwrap()
    }
}

// @code end
