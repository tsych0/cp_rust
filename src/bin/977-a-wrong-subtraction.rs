// Created by Ayush Biswas at 2025/05/16 17:04
// https://codeforces.com/problemset/problem/977/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    (
        [mut n, k] is [usize; 2]
    ) -> usize
    {
        for _ in 0..k {
            if n % 10 == 0 {
                n /= 10;
            } else {
                n -= 1;
            }
        }
        n
    }
}

// @code end
