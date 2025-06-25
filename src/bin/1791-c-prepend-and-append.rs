// Created by Ayush Biswas at 2025/06/03 21:30
// https://codeforces.com/problemset/problem/1791/C

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: usize,
        s: [01]
    ) -> usize {
        let (mut i, mut j) = (0, n - 1);
        while i < j {
            if s[i] + s[j] == 1 {
                i += 1;
                j -= 1;
            } else {
                break;
            }
        }
        j - i + 1
    }
}

// @code end
