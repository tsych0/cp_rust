// Created by Ayush Biswas at 2025/07/19 18:08
// https://cses.fi/problemset/task/1633
use cp_lib::*;

// @code begin
use cpio::*;
const MOD: usize = 10usize.pow(9) + 7;

sol! {
    fn solution(
        n: usize,
    ) -> usize {
        let mut ways = vec![0; n + 1];
        ways[0] = 1;

        for i in 0..n {
            for j in 1..=6 {
                if i + j <= n {
                    ways[i + j] += ways[i] % MOD;
                }
            }
        }

        // fn dice_combination(n: usize, ways: &mut Vec<usize>) {
        //     if n == 0 {
        //         ways[n] = 1;
        //         return;
        //     }

        //     for i in 1..=n.min(6) {
        //         if ways[n - i] == 0 {
        //             dice_combination(n - i, ways);
        //         }
        //         ways[n] += ways[n - i];
        //     }
        // }

        // dice_combination(n, &mut ways);
        ways[n] % MOD
    }
}

// @code end
