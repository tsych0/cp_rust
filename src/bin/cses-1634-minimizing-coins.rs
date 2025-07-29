// Created by Ayush Biswas at 2025/07/19 18:22
// https://cses.fi/problemset/task/1634
use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::min;

const MAX: u32 = 10u32.pow(9) + 7;

sol! {
    fn solution(
        [n, target]: [usize; 2],
        coins: [usize]
    ) -> CPResult<u32, i8> {
        // fn min_coins(target: usize, idx: usize, coins: &[usize]) -> usize {
        //     if target == 0 {
        //         return 0;
        //     }
        //     if idx == 0 {
        //         return MAX;
        //     }
        //     if coins[idx - 1] <= target {
        //         return min(
        //             1 + min_coins(target - coins[idx - 1], idx, coins),
        //             min_coins(target, idx - 1, coins)
        //         )
        //     }
        //     min_coins(target, idx - 1, coins)
        // }
        // min_coins(target, n, &coins)

        let mut res = vec![MAX; target + 1];
        res[0] = 0;

        // whenever you see this, we are copying the last layer
        // so why do that, just remove this line and 2d to 1d but the loops will be remain same
        // else {
        //   res[target][idx] = res[target][idx - 1];
        // }

        for target in 1..=target {
            for idx in 1..=n {
                if coins[idx - 1] <= target {
                    res[target] = min(
                        1 + res[target - coins[idx - 1]],
                        res[target]
                    );
                }
            }
        }

        if res[target] == MAX {
            Failure(-1)
        } else {
            Success(res[target])
        }
    }
}

// @code end
