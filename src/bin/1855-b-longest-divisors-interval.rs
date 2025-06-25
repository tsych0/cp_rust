// Created by Ayush Biswas at 2025/06/12 23:17
// https://codeforces.com/problemset/problem/1855/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize
    ) -> usize {
        (1..).take_while(|i| n % i == 0).count()
    }
}

// @code end
