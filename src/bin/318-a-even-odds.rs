// Created by Ayush Biswas at 2025/06/07 13:48
// https://codeforces.com/problemset/problem/318/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [n, k]: [usize; 2]
    ) -> usize {
        let odd_count = (n + 1) / 2;
        if k <= odd_count {
            2 * k - 1
        } else {
            2 * (k - odd_count)
        }
    }
}

// @code end
