// Created by Ayush Biswas at 2025/06/09 12:53
// https://codeforces.com/problemset/problem/1826/D

use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol_n! {
    fn solution(
        n: usize,
        b: [isize]
    ) -> isize {
        let left = b
            .iter()
            .enumerate()
            .scan(isize::MIN, |state, (i, bi)| {
                *state = (*state).max(bi + i as isize);
                Some(*state)
            })
            .collect_vec();
        let mut right = b
            .iter()
            .enumerate()
            .rev()
            .scan(isize::MIN, |state, (i, bi)| {
                *state = (*state).max(bi - i as isize);
                Some(*state)
            })
            .collect_vec();
        right.reverse();

        (1..n - 1).fold(0, |max_result, i|
             max_result.max(left[i - 1] + right[i + 1] + b[i])
        )
    }
}

// @code end
