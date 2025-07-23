// Created by Ayush Biswas at 2025/07/21 15:01
// https://cses.fi/problemset/task/1639
use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::min;

sol! {
    fn solution(
        n: [char],
        m: [char]
    ) -> usize {
        let n_len = n.len();
        let m_len = m.len();
        let mut res = vec![vec![0; m_len + 1]; n_len + 1];

        // The length of one is zero and other is i then i operations
        for i in 1..=n_len {
            res[i][0] = i;
        }
        for j in 1..=m_len {
            res[0][j] = j;
        }

        for i in 1..=n_len {
            for j in 1..=m_len {
                if n[i - 1] == m[j - 1] {
                    res[i][j] = res[i - 1][j - 1];
                } else {
                    res[i][j] = min(
                        1 + res[i - 1][j - 1],
                        min(
                            1 + res[i - 1][j],
                            1 + res[i][j - 1]
                        )
                    );
                }
            }
        }

        res[n_len][m_len]
    }
}

// @code end
