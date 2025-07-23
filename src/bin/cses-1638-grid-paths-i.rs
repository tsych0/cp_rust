// Created by Ayush Biswas at 2025/07/20 13:35
// https://cses.fi/problemset/task/1638
use cp_lib::*;

// @code begin
use cpio::*;
const MOD: usize = 10usize.pow(9) + 7;

sol! {
    fn solution(
        n: usize,
        grid: [[char]; n]
    ) -> usize {
        let mut res = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == '.' {
                    if i == 0 && j == 0 {
                        res[i][j] = 1;
                    } else if i == 0 {
                        res[i][j] = res[i][j - 1];
                    } else if j == 0 {
                        res[i][j] = res[i - 1][j];
                    } else {
                        res[i][j] = (res[i- 1][j] + res[i][j - 1]) % MOD;
                    }
                }
            }
        }

        res[n - 1][n - 1] % MOD
    }
}

// @code end
