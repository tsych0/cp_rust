// Created by Ayush Biswas at 2025/06/07 14:37
// https://codeforces.com/problemset/problem/96/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn (
        n: [01]
    ) -> bool {
        n.into_iter()
            .group_by(|&p| p)
            .map(|g| g.len())
            .max()
            .unwrap()
            >= 7
    }
}

// @code end
