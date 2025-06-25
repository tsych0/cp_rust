// Created by Ayush Biswas at 2025/06/15 12:28
// https://codeforces.com/problemset/problem/1617/B

use cp_lib::*;

use cpio::*;

sol! {
    fn (
        n: usize
    ) -> Words<usize> {
        let res = if n % 2 == 0 {
            vec![n / 2 - 1, n / 2, 1]
        } else {
            if n % 4 == 1 {
                vec![n / 2 - 1, n / 2 + 1, 1]
            } else {
                vec![n / 2 - 2, n / 2 + 2, 1]
            }
        };

        ListOf(res)
    }
}

// @code end
