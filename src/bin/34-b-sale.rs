// Created by Ayush Biswas at 2025/06/12 16:20
// https://codeforces.com/problemset/problem/34/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    (
        [n, m] is [usize; 2],
        a is [isize]
    ) -> isize
    {
        -a.into_iter()
            .filter(|&i| i < 0)
            .sorted()
            .take(m)
            .sum::<isize>()
    }
}
