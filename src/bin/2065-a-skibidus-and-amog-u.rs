// Created by Ayush Biswas at 2025/05/14 10:42
// https://codeforces.com/problemset/problem/2065/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        (mut s): String
    ) -> String {
        let len = s.len();
        s.replace_range(len - 2.., "i");
        s
    }
}

// @code end
