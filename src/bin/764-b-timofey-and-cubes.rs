// Created by Ayush Biswas at 2025/06/29 20:20
// https://codeforces.com/problemset/problem/764/B
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        (mut a): [isize]
    ) -> Words<isize> {
        for i in (0..n/2).step_by(2) {
            a.swap(i, n-i-1);
        }
        a.into()
    }
}

// @code end
