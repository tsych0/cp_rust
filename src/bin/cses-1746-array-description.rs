// Created by Ayush Biswas at 2025/07/20 18:42
// https://cses.fi/problemset/task/1746
use cp_lib::*;

// @code begin
use cpio::*;
const MOD: usize = 1_000_000_007;

sol! {
    fn solution(
        [n, m]: [usize; 2],
        a: [usize]
    ) -> usize {
        let mut res = vec![vec![0; m + 2]; 2];

        if a[0] == 0 {
            // brother end ki values fir bhi zero hi hongi na
            res[0][1..=m].fill(1);
        } else {
            res[0][a[0]] = 1;
        }

        for idx in 1..n {
            let curr = idx % 2;
            let prev = (idx - 1) % 2;

            res[curr].fill(0);
            for val in 1..=m {
                if a[idx] == val || a[idx] == 0 {
                    res[curr][val] =
                        (res[prev][val - 1] + res[prev][val] + res[prev][val + 1]) % MOD;
                }
            }
        }

        // tum bas sum lo baki sab handled hai
        res[1-n%2].iter().sum::<usize>() % MOD
    }
}

// @code end
