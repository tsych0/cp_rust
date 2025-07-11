// Created by Ayush Biswas at 2025/06/06 15:16
// https://codeforces.com/problemset/problem/1829/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        s: String
    ) -> usize {
        s.chars()
             .zip("codeforces".to_string().chars())
             .filter(|(a, b)| a != b)
             .count()
    }
}

// @code end
