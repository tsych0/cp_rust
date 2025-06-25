// Created by Ayush Biswas at 2025/05/14 18:17
// https://codeforces.com/problemset/problem/2057/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [n, m] is [usize; 2]
    ) -> usize
    {
        usize::max(n, m) + 1
    }
}

// @code end
