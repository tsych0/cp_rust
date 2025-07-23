// Created by Ayush Biswas at 2025/07/20 12:15
// https://cses.fi/problemset/task/1636
use cp_lib::*;

// @code begin
use cpio::*;
const MOD: usize = 10usize.pow(9) + 7;

sol! {
    fn solution(
        [n, target]: [usize; 2],
        coins: [usize]
    ) -> usize {
        let mut res = vec![0; target + 1];
        res[0] = 1;

        // but here res[idx][target] is representing the number of ways to produce the sum target
        // using only the first idx coins.

        for idx in 1..=n {
            for target in 1..=target {
                if coins[idx - 1] <= target {
                    res[target] += res[target - coins[idx - 1]] % MOD;
                }
            }
        }


        res[target] % MOD
    }
}

// @code end
