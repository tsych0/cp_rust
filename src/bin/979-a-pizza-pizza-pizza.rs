// Created by Ayush Biswas at 2025/07/07 16:32
// https://codeforces.com/problemset/problem/979/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
    ) -> usize {
        if n == 0 {
            return 0;
        }

        if (n+1) % 2 == 0 {
            (n+1)/2
        } else {
            n+1
        }
    }
}

// @code end
