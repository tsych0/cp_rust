// Created by Ayush Biswas at 2025/07/21 21:59
// https://cses.fi/problemset/task/3403
use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::max;

sol! {
    fn solution(
        [n, m]: [usize; 2],
        a: [usize],
        b: [usize]
    ) -> Lines<Words<usize>> {
        let mut res = vec![vec![0; m + 1]; n + 1];

        for i in 1..=n {
            for j in 1..=m {
                if a[i - 1] == b[j - 1] {
                    res[i][j] = max(
                        1 + res[i - 1][j - 1],
                        max(
                            res[i - 1][j],
                            res[i][j - 1]
                        )
                    );
                } else {
                    res[i][j] = max(
                            res[i - 1][j],
                            res[i][j - 1]
                    );
                }
            }
        }


        let mut i = n;
        let mut j = m;

        let mut result = vec![];
        while i > 0 && j > 0 {
            if res[i][j] == res[i - 1][j] {
                i -= 1;
            } else if res[i][j] == res[i][j - 1] {
                j -= 1;
            } else {
                i -= 1;
                j -= 1;
                result.push(a[i]);
            }
        }

        vec![vec![res[n][m]].into(), result.into_iter().rev().collect()].into()
    }
}

// @code end
