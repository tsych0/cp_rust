// Created by Ayush Biswas at 2025/05/14 10:42
// https://codeforces.com/problemset/problem/2065/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    (
        (mut s) is String
    ) -> String
    {
        let len = s.len();
        s.replace_range(len - 2.., "i");
        s
    }
}

// @code end
