// Created by Ayush Biswas at 2025/06/14 11:57
// https://codeforces.com/problemset/problem/1845/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [xa, ya]: [usize; 2],
        [xb, yb]: [usize; 2],
        [xc, yc]: [usize; 2]
    ) -> usize {
        let mut res = 0;
        if xa > xb && xa > xc || xa < xb && xa < xc {
            res += xa.abs_diff(xb).min(xa.abs_diff(xc));
        }

        if ya > yb && ya > yc || ya < yb && ya < yc {
            res += ya.abs_diff(yb).min(ya.abs_diff(yc));
        }

        res + 1
    }
}

// @code end
