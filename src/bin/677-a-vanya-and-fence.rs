// Created by Ayush Biswas at 2025/05/20 09:34
// https://codeforces.com/problemset/problem/677/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [n, h]: [usize; 2],
        a: [usize]
    ) -> usize {
        n + a.into_iter().filter(|&ai| ai > h).count()
    }
}

// @code end
