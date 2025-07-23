// Created by Ayush Biswas at 2025/07/22 11:20
// https://cses.fi/problemset/task/1745
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> Lines<Words<usize>> {
        let s = a.iter().sum();
        let mut res = vec![vec![false; s + 1]; 2];
        res[0][0] = true;
        res[1][0] = true;

        for (i, ai) in a.into_iter().sorted().enumerate() {
            let curr = i % 2;
            let prev = 1 - i % 2;

            for i in 0..=s {
                res[curr][i] = res[prev][i];
            }

            for i in 0..=s {
                if res[prev][i] && i + ai <= s {
                    res[curr][i + ai] = true;
                }
            }
        }

        let f = 1 - n%2;
        let res = (1..=s).filter(|&i| res[f][i]).collect_vec();
        vec![
            vec![res.len()].into(),
            res.into()
        ].into()
    }
}

// @code end
