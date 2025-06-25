// Created by Ayush Biswas at 2025/06/14 13:48
// https://codeforces.com/problemset/problem/1883/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::collections::HashMap;

sol! {
    (
        [_n, k] is [usize; 2],
        s is String
    ) -> bool
    {
        let count: HashMap<char, usize> = s
            .chars()
            .sorted()
            .group_by(|&x| x)
            .map(|g| (g[0], g.len()))
            .collect();
        count.values().filter(|&&v| v % 2 == 1).count() <= k + 1
    }
}

// @code end
