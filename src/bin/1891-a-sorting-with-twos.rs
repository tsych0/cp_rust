// Created by Ayush Biswas at 2025/06/06 11:47
// https://codeforces.com/problemset/problem/1891/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::io::Read;

sol! {
    (
        n is usize,
        a is [usize]
    ) -> bool
    {
        (1..=n.ilog2())
            .map(|i| 2usize.pow(i)..2usize.pow(i + 1).min(n))
            .map(|range| a.get(range).unwrap())
            .all(|ar| Itertools::is_sorted(ar.iter()))
    }
}

// @code end
