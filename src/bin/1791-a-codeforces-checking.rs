// Created by Ayush Biswas at 2025/05/19 22:25
// https://codeforces.com/problemset/problem/1791/A
use cp_lib::*;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;

sol! {
    (
        c is String
    ) -> bool
    {
        "codeforces".contains(&c)
    }
}

// @code end
