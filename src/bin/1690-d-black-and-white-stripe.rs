// Created by Ayush Biswas at 2025/07/06 13:48
// https://codeforces.com/problemset/problem/1690/D
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::{iter::once, ops::AddAssign, usize};

sol_n! {
    fn solution(
        [n, k]: [usize; 2],
        a: [char]
    ) -> usize {
        let r = a
            .into_iter()
            .scan(0, |acc, c| {
                if c == 'W' {
                    acc.add_assign(1);
                }
                Some(*acc)
            });
        let w = once(0).chain(r).collect_vec();

        (k..=n).fold(usize::MAX, |res, i| {
            res.min(w[i] - w[i-k])
        })
    }
}

// @code end
