// Created by Ayush Biswas at 2025/06/12 15:36
// https://codeforces.com/problemset/problem/1794/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: usize,
        (mut a): [usize]
    ) -> Words<usize> {
        for i in 0..n {
            if a[i] == 1 {
                a[i] = 2;
            }
        }
        for i in 1..n {
            while a[i] % a[i - 1] == 0 {
                a[i] += 1
            }
        }
        ListOf(a)
    }
}

// @code end
