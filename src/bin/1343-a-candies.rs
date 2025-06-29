// Created by Ayush Biswas at 2025/06/27 10:26
// https://codeforces.com/problemset/problem/1343/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
    ) -> usize {
        (2..).try_fold(1, |acc, _| {
            let s = (acc + 1) * 2 - 1;
            if n%s == 0 {
                Err(n/s)
            } else {
                Ok(s)
            }
        }).unwrap_err()
    }
}

// @code end
