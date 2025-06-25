// Created by Ayush Biswas at 2025/05/17 09:40
// https://codeforces.com/problemset/problem/546/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [k, n, w] is [isize; 3]
    ) -> isize
    {
        (k * ((w * (w + 1)) / 2) - n).max(0)
    }
}

// @code end
