// Created by Ayush Biswas at 2025/05/17 15:49
// https://codeforces.com/problemset/problem/271/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashSet;

sol! {
    fn (
        n: usize
    ) -> usize {
        (n + 1..)
            .find(|&m| m.to_string().chars().collect::<HashSet<_>>().len() == 4)
            .unwrap()
    }
}

// @code end
