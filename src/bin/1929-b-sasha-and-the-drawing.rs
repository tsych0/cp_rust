// Created by Ayush Biswas at 2025/06/03 14:25
// https://codeforces.com/problemset/problem/1929/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (
        [n, k] is [usize; 2]
    ) -> usize
    {
        let total_diagonals = 4 * n - 2;
        let diff = total_diagonals - k;
        if diff < 2 {
            (2 * n) - diff
        } else {
            (2 * n) - 2 - (diff - 2) / 2
        }
    }
}

// @code end
