// Created by Ayush Biswas at 2025/06/12 16:20
// https://codeforces.com/problemset/problem/34/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn (
        [_n, m]: [usize; 2],
        a: [isize]
    ) -> isize {
        -a.into_iter()
            .filter(|&i| i < 0)
            .sorted()
            .take(m)
            .sum::<isize>()
    }
}
