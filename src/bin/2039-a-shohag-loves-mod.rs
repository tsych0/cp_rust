// Created by Ayush Biswas at 2025/05/14 20:54
// https://codeforces.com/problemset/problem/2039/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: u8
    ) -> Words<u8> {
        (0..n).map(|i| 2 * i + 1).collect()
    }
}

// @code end
