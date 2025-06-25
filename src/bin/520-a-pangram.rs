// Created by Ayush Biswas at 2025/06/03 20:28
// https://codeforces.com/problemset/problem/520/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn (
        _n: usize,
        s: String
    ) -> bool {
        s.to_lowercase().chars().unique().count() == 26
    }
}

// @code end
