// Created by Ayush Biswas at 2025/07/06 12:10
// https://codeforces.com/problemset/problem/742/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
    ) -> usize {
        if n == 0 {
            return 1;
        }
        match n%4 {
            1 => 8,
            2 => 4,
            3 => 2,
            0 => 6,
            _ => 0
        }
    }
}

// @code end
