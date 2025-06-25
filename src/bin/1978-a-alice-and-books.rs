// Created by Ayush Biswas at 2025/05/20 11:09
// https://codeforces.com/problemset/problem/1978/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        n is usize,
        a is [usize]
    ) -> usize
    {
        a.get(0..n - 1).unwrap().iter().max().unwrap() + a.last().unwrap()
    }
}

// @code end
