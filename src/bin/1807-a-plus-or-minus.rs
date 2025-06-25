// Created by Ayush Biswas at 2025/06/03 20:01
// https://codeforces.com/problemset/problem/1807/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [a, b, c]: [isize; 3]
    ) -> char {
        if a + b == c {
            '+'
        } else {
            '-'
        }
    }
}
// @code end
