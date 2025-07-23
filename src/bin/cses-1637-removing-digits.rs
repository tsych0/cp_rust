// Created by Ayush Biswas at 2025/07/20 13:29
// https://cses.fi/problemset/task/1637
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashSet;

sol! {
    fn solution(
        n: usize,
    ) -> usize {
        fn digits(mut n: usize) -> HashSet<usize> {
            let mut res = HashSet::new();
            while n > 0 {
                res.insert(n%10);
                n /= 10;
            }
            res
        }
        let mut res = vec![usize::MAX; n + 1];
        res[0] = 0;

        for i in 1..=n {
            for dig in digits(i) {
                res[i] = res[i].min(res[i - dig]);
            }
            res[i] += 1;
        }

        res[n]
    }
}

// @code end
