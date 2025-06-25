// Created by Ayush Biswas at 2025/06/06 15:53
// https://codeforces.com/problemset/problem/1886/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize
    ) -> Words<String> {
        for [i, j, k] in [[1, 1, 1], [2, 2, 2], [1, 2, 2], [2, 1, 1]] {
            let rem_n = n - (i + j + k);
            if rem_n % 3 != 0 {
                continue;
            }
            let sum_ijk = rem_n / 3;
            if i == j {
                if sum_ijk >= 3 {
                    return vec![
                        "YES".into(),
                        words_of(vec![i, 3 + j, (sum_ijk - 1) * 3 + k]).to_string(),
                    ]
                    .into();
                }
            } else {
                if sum_ijk >= 1 {
                    return vec![
                        "YES".into(),
                        words_of(vec![i, j, sum_ijk * 3 + k]).to_string(),
                    ]
                    .into();
                }
            }
        }
        ListOf(vec!["NO".into()])
    }
}
// @code end
