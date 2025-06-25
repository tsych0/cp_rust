// Created by Ayush Biswas at 2025/06/13 11:48
// https://codeforces.com/problemset/problem/1818/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: isize
    ) -> Words<isize> {
        let res = if n == 1 {
            vec![1]
        } else if n % 2 == 1 {
            vec![-1]
        } else {
            (1..=n / 2)
                .map(|i| vec![2 * i, 2 * i - 1])
                .collect::<Vec<_>>()
                .concat()
        };

        ListOf(res)
    }
}

// @code end
