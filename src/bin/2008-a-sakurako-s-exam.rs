// Created by Ayush Biswas at 2025/05/17 16:12
// https://codeforces.com/problemset/problem/2008/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [a, b]: [usize; 2]
    ) -> bool {
        a % 2 == 0 && (b % 2 == 0 || a >= 2)
    }
}

// @code end
