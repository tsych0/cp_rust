// Created by Ayush Biswas at 2025/07/22 11:08
// https://cses.fi/problemset/task/1744
use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::min;

sol! {
    fn solution(
        [n, m]: [usize; 2]
    ) -> usize {
        let mut res = vec![vec![usize::MAX; m + 1]; n + 1];

        for i in 0..=n {
            for j in 0..=m {
                if i == j {
                    res[i][j] = 0;
                    continue;
                }
                for c in 1..i {
                    res[i][j] = min(
                        res[i][j],
                        1 + res[i - c][j] + res[c][j]
                    );
                }
                for c in 1..j {
                    res[i][j] = min(
                        res[i][j],
                        1 + res[i][j - c] + res[i][c]
                    );
                }
            }
        }

        res[n][m]
    }
}

// @code end
