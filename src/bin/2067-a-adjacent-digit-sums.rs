// Created by Ayush Biswas at 2025/05/14 10:38
// https://codeforces.com/problemset/problem/2067/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (
        [x, y] is [usize; 2]
    ) -> bool
    {
        if x > y {
            (x - y + 1) % 9 == 0
        } else {
            (y - x) == 1
        }
    }
}

// @code end
