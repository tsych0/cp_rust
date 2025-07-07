// Created by Ayush Biswas at 2025/07/07 11:06
// https://codeforces.com/problemset/problem/1791/D
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::collections::HashSet;

sol_n! {
    fn solution(n: usize, s: String) -> usize {
        let prefix = s
            .chars()
            .scan(HashSet::new(), |state, c| {
                state.insert(c);
                Some(state.len())
            })
            .collect_vec();
        let suffix = s
            .chars()
            .rev()
            .scan(HashSet::new(), |state, c| {
                state.insert(c);
                Some(state.len())
            })
            .collect_vec();

        (0..n-1).map(|i| prefix[i] + suffix[n - i - 2]).max().unwrap()
    }
}

// @code end
