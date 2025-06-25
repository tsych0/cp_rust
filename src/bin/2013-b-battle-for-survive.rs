// Created by Ayush Biswas at 2025/06/11 15:48
// https://codeforces.com/problemset/problem/2013/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: usize,
        a: [isize]
    ) -> isize {
        a[n - 1] - a[n - 2] + a.into_iter().take(n - 2).sum::<isize>()
    }
}

// @code end
