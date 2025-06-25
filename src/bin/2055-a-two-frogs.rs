// Created by Ayush Biswas at 2025/05/14 20:41
// https://codeforces.com/problemset/problem/2055/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [_n, a, b] is [i8; 3]
    ) -> bool
    {
        (a - b).abs() % 2 == 0
    }
}

// @code end
