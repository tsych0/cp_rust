// Created by Ayush Biswas at 2025/06/09 12:33
// https://codeforces.com/problemset/problem/2035/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;
use std::iter::repeat;

sol! {
    (
        n is usize
    ) -> String
    {
        if n % 2 == 0 {
            repeat('3').take(n - 2).collect::<String>() + "66"
        } else if n >= 5 {
            repeat('3').take(n - 5).collect::<String>() + "36366"
        } else {
            "-1".into()
        }
    }
}

// @code end
