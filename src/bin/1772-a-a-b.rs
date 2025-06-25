// Created by Ayush Biswas at 2025/06/04 10:54
// https://codeforces.com/problemset/problem/1772/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        s: String
    ) -> usize {
        s.split('+').map(|x| x.parse::<usize>().unwrap()).sum()
    }
}

// @code end
