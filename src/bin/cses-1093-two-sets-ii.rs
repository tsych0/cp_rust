// Created by Ayush Biswas at 2025/07/22 12:38
// https://cses.fi/problemset/task/1093
use cp_lib::*;

// @code begin
use cpio::*;
const MOD: usize = 10usize.pow(9) + 7;

sol! {
    fn solution(
        n: usize,
    ) -> usize {
        let s = (n * (n + 1))/2;
        if s % 2 != 0 {
            return 0;
        }
        let required_sum = s/2;

        let mut res = vec![vec![0; s + 1]; n + 1];

        for i in 0..=n {
            res[i][0] = 1;
        }

        for i in 1..=n {
            for sum in 1..=s {
                if sum >= i {
                    res[i][sum] = (res[i - 1][sum] + res[i - 1][sum - i]) % MOD;
                } else {
                    res[i][sum] = res[i - 1][sum] % MOD;
                }
            }
        }

        (res[n][required_sum] % MOD * 500000004) % MOD
    }
}

// @code end
