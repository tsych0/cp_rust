// Created by Ayush Biswas at 2025/06/12 11:55
// https://codeforces.com/problemset/problem/1850/D

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol_n! {
    fn solution(
        [n, k]: [usize; 2],
        (mut a): [usize]
    ) -> usize {
        a.sort();
        let mut r = vec![0; n];
        for i in 1..n {
            if a[i] - a[i - 1] > k {
                r[i] = r[i - 1] + 1;
            } else {
                r[i] = r[i - 1];
            }
        }
        n - r
            .into_iter()
            .group_by(|&i| i)
            .map(|g| g.len())
            .max()
            .unwrap()
    }
}

// @code end
