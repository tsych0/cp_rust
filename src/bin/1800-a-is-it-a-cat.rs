// Created by Ayush Biswas at 2025/06/04 10:32
// https://codeforces.com/problemset/problem/1800/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    (
        _n is usize,
        s is String
    ) -> bool
    {
        s.to_lowercase()
            .chars()
            .group_by(|&c| c)
            .map(|g| g[0])
            .collect::<String>()
            == "meow"
    }
}

// @code end
