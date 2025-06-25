// Created by Ayush Biswas at 2025/06/07 14:31
// https://codeforces.com/problemset/problem/2106/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [n, x]: [usize; 2]
    ) -> Words<usize> {
        let mut r: Vec<usize> = (0..n).filter(|&i| i != x).collect();
        if x < n {
            r.push(x);
        }
        ListOf(r)
    }
}
// @code end
