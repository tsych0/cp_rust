// Created by Ayush Biswas at 2025/07/20 12:09
// https://cses.fi/problemset/task/1635
use cp_lib::*;

// @code begin
use cpio::*;
const MOD: usize = 10usize.pow(9) + 7;

sol! {
    fn solution(
        [n, target]: [usize; 2],
        coins: [usize]
    ) -> Result<usize> {
        // fn num_ways(target: usize, idx: usize, coins: &[usize]) -> usize {
        //     if target == 0 {
        //         return 1;
        //     }
        //     if idx == 0 {
        //         return 0;
        //     }
        //     if coins[idx - 1] <= target {
        //         return
        //             num_ways(target - coins[idx - 1], idx, coins) +
        //             num_ways(target, idx - 1, coins)
        //         )
        //     }
        //     num_ways(target, idx - 1, coins)
        // }
        // num_ways(target, n, &coins)

        let mut res = vec![0; target + 1];
        res[0] = 1;

        // whenever you see this, we are copying the last layer
        // so why do that, just remove this line and 2d to 1d but the loops will be remain same
        // else {
        //   res[target][idx] = res[target][idx - 1];
        // }
        //
        // here res[i] is representing ways to produce the sum i using all the coins

        for target in 1..=target {
            for idx in 1..=n {
                if coins[idx - 1] <= target {
                    res[target] += res[target - coins[idx - 1]] % MOD;
                }
            }
        }


        Ok(res[target] % MOD)
    }
}

// @code end
