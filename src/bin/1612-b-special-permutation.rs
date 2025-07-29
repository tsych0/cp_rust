// Created by Ayush Biswas at 2025/06/18 15:12
// https://codeforces.com/problemset/problem/1612/B

use cp_lib::*;

use cpio::*;

sol_n! {
    fn solution(
        [n, a, b]: [usize; 3],
    ) -> CPResult<Words<usize>, i8> {
        let mut res = vec![0; n];
        res[0] = a;
        res[n - 1] = b;
        for (i, c) in (1..=n).filter(|&x| x != a && x != b).rev().enumerate() {
            res[i + 1] = c;
        }
        if res[0..n / 2].into_iter().min().unwrap() == &a
            && res[n / 2..n].into_iter().max().unwrap() == &b {
            Success(ListOf(res))
        } else {
            Failure(-1)
        }
    }
}

// @code end
