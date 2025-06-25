// Created by Ayush Biswas at 2025/06/06 15:27
// https://codeforces.com/problemset/problem/996/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        n is usize
    ) -> usize
    {
        let denominations = [100, 20, 10, 5, 1];
        denominations
            .into_iter()
            .fold((0, n), |(res, rem), d| (res + rem / d, rem % d))
            .0
    }
}

// @code end
