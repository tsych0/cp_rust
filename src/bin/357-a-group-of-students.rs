// Created by Ayush Biswas at 2025/07/07 16:39
// https://codeforces.com/problemset/problem/357/A
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::ops::AddAssign;

sol! {
    fn solution(
        n: usize,
        c: [usize],
        [x, y]: [usize; 2]
    ) -> usize {
        let prefix = c.iter().scan(0, |acc, ci| {
            acc.add_assign(ci);
            Some(*acc)
        })
        .collect_vec();
        let suffix = c.iter().rev().scan(0, |acc, ci| {
            acc.add_assign(ci);
            Some(*acc)
        })
        .collect_vec();
        (0..n-1)
            .find(|&i| {
                prefix[i] >= x && prefix[i] <= y &&
                suffix[n-i-2] >= x && suffix[n-i-2] <= y
            }).map(|i| i+2).unwrap_or(0)
    }
}

// @code end
