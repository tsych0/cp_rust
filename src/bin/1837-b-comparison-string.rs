// Created by Ayush Biswas at 2025/06/10 22:14
// https://codeforces.com/problemset/problem/1837/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn (
        n: usize,
        s: String
    ) -> usize {
        s.chars().group_by(|&c| c).map(|g| g.len()).max().unwrap() + n % 2
    }
}

// @code end
