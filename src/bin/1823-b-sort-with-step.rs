// Created by Ayush Biswas at 2025/06/20 15:09
// https://codeforces.com/problemset/problem/1823/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::collections::HashSet;

sol! {
    fn (
        [n, k]: [usize; 2],
        p: [usize]
    ) -> isize {
        let x = p
            .into_iter()
            .enumerate()
            .sorted_by_key(|(i, _)| i % k)
            .group_by(|(i, _)| i % k)
            .map(|g| g.into_iter().map(|(_, j)| j).collect::<HashSet<_>>());
        let y = (1..=n)
            .enumerate()
            .sorted_by_key(|(i, _)| i % k)
            .group_by(|(i, _)| i % k)
            .map(|g| g.into_iter().map(|(_, j)| j).collect::<HashSet<_>>());

        let faults = x.zip(y).map(|(xi, yi)| xi.difference(&yi).count()).sum();

        match faults {
            0 => 0,
            2 => 1,
            _ => -1,
        }
    }
}

// @code end
