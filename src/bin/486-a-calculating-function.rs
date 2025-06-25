// Created by Ayush Biswas at 2025/05/20 11:12
// https://codeforces.com/problemset/problem/486/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    (        n is isize
    ) -> isize
    {
        if n % 2 == 0 {
            (n + 1) / 2
        } else {
            -(n + 1) / 2
        }
    }
}

// @code end
