// Created by Ayush Biswas at 2025/06/03 17:01
// https://codeforces.com/problemset/problem/1918/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [n, m] is [usize; 2]
    ) -> usize
    {
        n * (m / 2)
    }
}

// @code end
