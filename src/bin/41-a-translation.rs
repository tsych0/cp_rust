// Created by Ayush Biswas at 2025/05/17 16:39
// https://codeforces.com/problemset/problem/41/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        s is String,
        t is String
    ) -> bool
    {
        s.chars().rev().collect::<String>() == t
    }
}
// @code end
