// Created by Ayush Biswas at 2025/05/19 09:17
// https://codeforces.com/problemset/problem/2001/B
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize
    ) -> Option<Words<usize>> {
        if n % 2 == 0 {
            None
        } else {
            let second_part = (1..=n).take((n + 1) / 2).collect::<Vec<_>>();
            let first_part = (1..=n).rev().take(n / 2).collect::<Vec<_>>();
            Some([first_part.as_slice(), second_part.as_slice()]
                .concat()
                .into())
        }
    }
}

// @code end
