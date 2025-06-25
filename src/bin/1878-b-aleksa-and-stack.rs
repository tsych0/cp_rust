// Created by Ayush Biswas at 2025/06/07 12:43
// https://codeforces.com/problemset/problem/1878/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (        n is usize
    ) -> Words<usize>
    {
        (1..=n).map(|i| 2 * i - 1).collect()
    }
}

// @code end
