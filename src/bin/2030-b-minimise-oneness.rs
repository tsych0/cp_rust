// Created by Ayush Biswas at 2025/05/16 12:21
// https://codeforces.com/problemset/problem/2030/B
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize
    ) -> String {
        vec!['0'; n - 1].into_iter().collect::<String>() + "1"
    }
}

// @code end
