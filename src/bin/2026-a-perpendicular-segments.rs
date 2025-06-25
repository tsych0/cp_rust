// Created by Ayush Biswas at 2025/06/10 22:05
// https://codeforces.com/problemset/problem/2026/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [x, y, k]: [usize; 3]
    ) -> Lines<Words<usize>> {
        if x >= k && y >= k {
            vec![vec![0, 0, x, 0].into(), vec![0, 0, 0, y].into()]
        } else {
            let z = x.min(y);
            vec![vec![0, 0, z, z].into(), vec![0, z, z, 0].into()]
        }
        .into()
    }
}

// @code end
