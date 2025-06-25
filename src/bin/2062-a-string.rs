// Created by Ayush Biswas at 2025/05/13 22:27
// https://codeforces.com/problemset/problem/2062/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        s is [01]
    ) -> u8
    {
        s.into_iter().sum()
    }
}

// @code end
