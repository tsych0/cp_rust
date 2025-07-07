// Created by Ayush Biswas at 2025/06/07 21:30
// https://codeforces.com/problemset/problem/1807/D

use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::{iter::once, ops::AddAssign};

sol_n! {
    fn solution(
        [_n, q]: [usize; 2],
        a: [usize],
        queries: [[usize]; q]
    ) -> Lines<bool> {
        let prefix_sum = once(0)
            .chain(a.into_iter().scan(0, |acc, x| {
                acc.add_assign(x);
                Some(*acc)
            })).collect_vec();

        let a_sum = prefix_sum.last().unwrap();

        queries
            .into_iter()
            .map(|lrk| {
                let [l, r, k] = lrk.try_into().unwrap();
                let final_sum = a_sum - (prefix_sum[r] - prefix_sum[l - 1]) + (r - l + 1) * k;
                final_sum % 2 == 1
            })
            .collect()
    }
}

// @code end
