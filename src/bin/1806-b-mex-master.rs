// Created by Ayush Biswas at 2025/06/13 12:07
// https://codeforces.com/problemset/problem/1806/B

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> u8 {
        if a.iter().filter(|&&i| i == 0).count() > (n + 1) / 2 {
            if a.into_iter().max().unwrap() == 1 {
                2
            } else {
                1
            }
        } else {
            0
        }
    }
}

// @code end
