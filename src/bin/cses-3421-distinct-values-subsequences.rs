// Created by Ayush Biswas at 2025/07/17 22:48
// https://cses.fi/problemset/task/3421
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use modint::ModInt1000000007;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> u32 {
        (a
            .into_iter()
            .sorted()
            .group_by(|&ai| ai)
            .map(|g| g.len() + 1)
            .fold(ModInt1000000007::new(1), |acc, s| acc * ModInt1000000007::new(s))
        - ModInt1000000007::new(1)
        ).val()
    }
}

// @code end
