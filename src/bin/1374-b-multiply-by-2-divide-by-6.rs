// Created by Ayush Biswas at 2025/06/18 14:34
// https://codeforces.com/problemset/problem/1374/B

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize
    ) -> CPResult<usize, i8> {
        let (two_count, rem) = (0..)
            .try_fold(n, |acc, i| {
                if acc % 2 == 0 {
                    Ok(acc / 2)
                } else {
                    Err((i, acc))
                }
            })
            .unwrap_err();
        let (three_count, rem) = (0..)
            .try_fold(rem, |acc, i| {
                if acc % 3 == 0 {
                    Ok(acc / 3)
                } else {
                    Err((i, acc))
                }
            })
            .unwrap_err();
        if two_count > three_count || rem != 1 {
            Err(-1)
        } else {
            Ok(2 * three_count - two_count)
        }.into()
    }
}

// @code end
