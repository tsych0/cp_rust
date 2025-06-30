// Created by Ayush Biswas at 2025/06/30 15:24
// https://codeforces.com/problemset/problem/246/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: isize,
    ) -> Words<isize> {
        if n < 3 {
            vec![-1].into()
        } else {
            (1..=n).rev().collect()
        }
    }
}

// @code end
