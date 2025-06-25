// Created by Ayush Biswas at 2025/06/18 15:58
// https://codeforces.com/problemset/problem/1875/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        [a, b, _n] is [usize; 3],
        x is [usize]
    ) -> usize
    {
        x.into_iter().map(|xi| xi.min(a - 1)).sum::<usize>() + b
    }
}

// @code end
