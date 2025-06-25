// Created by Ayush Biswas at 2025/06/04 10:50
// https://codeforces.com/problemset/problem/1902/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    (
        n is usize,
        s is [01]
    ) -> bool
    {
        let s = s.into_iter().unique().collect::<Vec<_>>();
        s != vec![1]
    }
}

// @code end
