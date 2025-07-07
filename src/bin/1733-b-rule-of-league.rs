// Created by Ayush Biswas at 2025/06/14 16:01
// https://codeforces.com/problemset/problem/1733/B

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, x, y]: [usize; 3]
    ) -> Option<Words<usize>> {
        let k = x + y;
        if x != 0 && y != 0 || x + y == 0 || ((n - 1) % k != 0) {
            None
        } else {
            ListOf(
                vec![
                    vec![1; k as usize],
                    (k..n - 1).map(|j| 2 + (j / k) * k).collect(),
                ]
                .concat(),
            ).into()
        }
    }
}

// @code end
