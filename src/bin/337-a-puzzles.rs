// Created by Ayush Biswas at 2025/06/12 15:11
// https://codeforces.com/problemset/problem/337/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;
use std::usize;

sol! {
    fn (
        [n, m]: [usize; 2],
        (mut f): [usize]
    ) -> usize {
        f.sort();
        let mut res = usize::MAX;
        for i in 0..m - n + 1 {
            res = res.min(f[i + n - 1] - f[i])
        }
        res
    }
}

// @code end
