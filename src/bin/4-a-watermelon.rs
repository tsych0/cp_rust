// Created by Ayush Biswas at 2025/05/13 23:06
// https://codeforces.com/problemset/problem/4/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: usize
    ) -> bool {
        n > 2 && n % 2 == 0
    }
}

// @code end
