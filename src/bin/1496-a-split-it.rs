// Created by Ayush Biswas at 2025/06/19 11:16
// https://codeforces.com/problemset/problem/1496/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (
        [n, k] is [usize; 2],
        s is String
    ) -> bool
    {
        let s_rev = s.chars().rev().collect::<String>();
        n > 2 * k && s[..k] == s_rev[..k] && s[n - k..] == s_rev[n - k..]
    }
}

// @code end
