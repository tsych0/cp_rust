// Created by Ayush Biswas at 2025/05/20 09:34
// https://codeforces.com/problemset/problem/677/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (
        [n, h] is [usize; 2],
        a is [usize]
    ) -> usize
    {
        n + a.into_iter().filter(|&ai| ai > h).count()
    }
}

// @code end
