// Created by Ayush Biswas at 2025/05/20 11:20
// https://codeforces.com/problemset/problem/1969/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
        p: [usize]
    ) -> usize {
        for i in 0..n {
            if i == p[p[i] - 1] - 1 {
                return 2;
            }
        }

        return 3;
    }
}

// @code end
