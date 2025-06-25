// Created by Ayush Biswas at 2025/06/12 23:25
// https://codeforces.com/problemset/problem/1918/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    (
        _n is usize,
        a is [usize],
        b is [usize]
    ) -> Lines<Words<usize>>
    {
        let (a, b): (Vec<_>, Vec<_>) = a.into_iter().zip(b.into_iter()).sorted().unzip();
        vec![a.into(), b.into()].into()
    }
}
// @code end
