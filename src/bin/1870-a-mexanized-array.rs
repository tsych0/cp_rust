// Created by Ayush Biswas at 2025/06/07 13:19
// https://codeforces.com/problemset/problem/1870/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [n, k, x] is [isize; 3]
    ) -> isize
    {
        if k > n || k > x + 1 {
            return -1;
        }
        if x == k {
            (0..k).sum::<isize>() + (n - k) * (x - 1)
        } else {
            (0..k).sum::<isize>() + (n - k) * x
        }
    }
}

// @code end
