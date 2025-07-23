// Created by Ayush Biswas at 2025/07/20 13:49
// https://cses.fi/problemset/task/1158
use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::max;

sol! {
    fn solution([n, budget]: [usize; 2], prices: [usize], pages: [usize]) -> usize {
        let mut res = vec![vec![0; 2]; budget + 1];
        let budget = budget;

        for idx in 1..=n {
            for b in 1..=budget {
                let curr = idx % 2;
                let prev = (idx - 1) % 2;

                if prices[idx - 1] <= b {
                    res[b][curr] = max(
                        pages[idx - 1] + res[b - prices[idx - 1]][prev],
                        res[b][prev],
                    );
                } else {
                    res[b][curr] = res[b][prev];
                }
            }
        }

        res[budget][n % 2]
    }
}

// @code end
