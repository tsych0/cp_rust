// Created by Ayush Biswas at 2025/06/03 16:33
// https://codeforces.com/problemset/problem/1328/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [a, b]: [usize; 2]
    ) -> usize {
        if a % b == 0 {
            0
        } else {
            b - a % b
        }
    }
}

// @code end
