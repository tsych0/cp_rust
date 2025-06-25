// Created by Ayush Biswas at 2025/06/12 15:23
// https://codeforces.com/problemset/problem/1954/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [n, m, k]: [isize; 3]
    ) -> bool {
        k < (n - (n + m - 1) / m)
    }
}

// @code end
