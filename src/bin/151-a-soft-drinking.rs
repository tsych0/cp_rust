// Created by Ayush Biswas at 2025/06/07 13:01
// https://codeforces.com/problemset/problem/151/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [n, k, l, c, d, p, nl, np]: [usize; 8]
    ) -> usize {
        ((k * l) / nl).min(c * d).min(p / np) / n
    }
}
// @code end
