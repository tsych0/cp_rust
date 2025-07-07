// Created by Ayush Biswas at 2025/07/04 12:30
// https://codeforces.com/problemset/problem/139/A
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::ops::AddAssign;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        let prefix_sum = a.into_iter().scan(0, |acc, x| {
                acc.add_assign(x);
                Some(*acc)
            }).collect_vec();

        let ws = prefix_sum[6];

        let ks = prefix_sum.into_iter().map(|pi| {
            n.saturating_sub(pi).div_ceil(ws)
        }).collect_vec();

        let kmin = ks.iter().min().unwrap().clone();

        ks.into_iter().enumerate().find(|&(_, ki)| ki == kmin).unwrap().0 + 1
    }
}

// @code end
