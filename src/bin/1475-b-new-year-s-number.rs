// Created by Ayush Biswas at 2025/06/13 12:41
// https://codeforces.com/problemset/problem/1475/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    (
        n is usize
    ) -> bool
    {
        n / 2020 >= n % 2020
    }
}

// @code end
