// Created by Ayush Biswas at 2025/06/19 10:50
// https://codeforces.com/problemset/problem/1525/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: usize,
        a: [usize]
    ) -> u8 {
        if a == (1..=n).collect::<Vec<_>>() {
            0
        } else if a[0] == 1 || a[n - 1] == n {
            1
        } else if a[0] != n || a[n - 1] != 1 {
            2
        } else {
            3
        }
    }
}

// @code end
