// Created by Ayush Biswas at 2025/05/14 17:47
// https://codeforces.com/problemset/problem/112/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        a is String,
        b is String
    ) -> i8
    {
        use std::cmp::Ordering::*;
        match a.to_lowercase().cmp(&b.to_lowercase()) {
            Greater => 1,
            Equal => 0,
            Less => -1,
        }
    }
}

// @code end
