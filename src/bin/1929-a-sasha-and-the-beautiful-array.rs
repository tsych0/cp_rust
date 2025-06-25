// Created by Ayush Biswas at 2025/06/03 15:17
// https://codeforces.com/problemset/problem/1929/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        _n is usize,
        a is [usize]
    ) -> usize
    {
        a.iter().max().unwrap() - a.iter().min().unwrap()
    }
}
// @code end
