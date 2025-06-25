// Created by Ayush Biswas at 2025/06/14 12:37
// https://codeforces.com/problemset/problem/1758/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize
    ) -> Words<usize> {
        if n % 2 == 0 {
            ListOf(vec![vec![1, 3], vec![2; n - 2]].concat())
        } else {
            ListOf(vec![69; n])
        }
    }
}

// @code end
