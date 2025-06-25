// Created by Ayush Biswas at 2025/06/03 21:45
// https://codeforces.com/problemset/problem/1914/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, k]: [usize; 2]
    ) -> Words<usize> {
        let cut_point = n - k;

        [
            (cut_point..=n).collect::<Vec<_>>(),
            (1..cut_point).rev().collect::<Vec<_>>(),
        ]
        .concat()
        .into()
    }
}

// @code end
