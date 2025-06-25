// Created by Ayush Biswas at 2025/06/13 15:15
// https://codeforces.com/problemset/problem/313/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::max;
use std::io::Read;

sol! {
    (
        n is isize
    ) -> isize
    {
        if n >= 0 {
            n
        } else {
            let l = n % 10;
            max((n / 100) * 10 + l, n / 10)
        }
    }
}

// @code end
