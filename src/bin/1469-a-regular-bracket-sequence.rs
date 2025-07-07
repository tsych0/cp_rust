// Created by Ayush Biswas at 2025/07/05 11:28
// https://codeforces.com/problemset/problem/1469/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        s: [char]
    ) -> bool {
        s.len()%2 == 0 && s[0] != ')' && *s.last().unwrap() != '('
    }
}

// @code end
