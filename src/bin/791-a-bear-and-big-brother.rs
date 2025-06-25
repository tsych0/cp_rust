// Created by Ayush Biswas at 2025/05/15 13:32
// https://codeforces.com/problemset/problem/791/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [mut a, mut b]: [usize; 2]
    ) -> usize {
        for i in 1.. {
            a *= 3;
            b *= 2;
            if a > b {
                return i;
            }
        }
        0
    }
}

// @code end
