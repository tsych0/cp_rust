// Created by Ayush Biswas at 2025/06/04 10:41
// https://codeforces.com/problemset/problem/1905/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::max;

sol! {
    (
        [n, m] is [usize; 2]
    ) -> usize
    {
        max(n, m)
    }
}

// @code end
