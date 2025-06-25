// Created by Ayush Biswas at 2025/06/12 13:14
// https://codeforces.com/problemset/problem/1992/C

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, m, _k]: [usize; 3]
    ) -> Words<usize> {
        vec![
            (m + 1..=n).rev().collect::<Vec<_>>(),
            (1..=m).collect::<Vec<_>>(),
        ]
        .concat()
        .into()
    }
}

// @code end
