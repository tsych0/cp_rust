// Created by Ayush Biswas at 2025/05/20 08:50
// https://codeforces.com/problemset/problem/228/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashSet;

sol! {
    fn solution(
        shoes: [usize]
    ) -> usize {
        4 - shoes.into_iter().collect::<HashSet<_>>().len()
    }
}

// @code end
