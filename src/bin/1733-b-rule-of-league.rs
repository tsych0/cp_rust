// Created by Ayush Biswas at 2025/06/14 16:01
// https://codeforces.com/problemset/problem/1733/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [n, x, y]: [isize; 3]
    ) -> Words<isize> {
        let k = x + y;
        if x != 0 && y != 0 || x + y == 0 || ((n - 1) % k != 0) {
            ListOf(vec![-1])
        } else {
            ListOf(
                vec![
                    vec![1; k as usize],
                    (k..n - 1).map(|j| 2 + (j / k) * k).collect(),
                ]
                .concat(),
            )
        }
    }
}

// @code end
