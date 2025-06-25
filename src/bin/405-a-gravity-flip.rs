// Created by Ayush Biswas at 2025/06/09 12:40
// https://codeforces.com/problemset/problem/405/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::io::Read;

sol! {
    (
        n is usize,
        a is [usize]
    ) -> Words<usize>
    {
        a.into_iter().sorted().collect()
    }
}

// @code end
