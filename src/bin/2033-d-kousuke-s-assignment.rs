// Created by Ayush Biswas at 2025/07/01 16:56
// https://codeforces.com/problemset/problem/2033/D
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::{collections::HashMap, iter::once, ops::AddAssign};

sol_n! {
    fn solution(
        _: usize,
        a: [isize]
    ) -> usize {
        let prefix_sum = once(0)
            .chain(a.into_iter().scan(0, |acc, x| {
                acc.add_assign(x);
                Some(*acc)
            })).collect_vec();

        let zero_segments = prefix_sum
            .into_iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, pi)| {
                acc.entry(pi).or_insert(vec![]).push(i);
                acc
            })
            .values()
            .map(|v| {
                v.windows(2).map(|w| (w[0], w[1])).collect_vec()
            })
            .collect_vec()
            .concat()
            .into_iter()
            .sorted_by_key(|&(_, i)| i);

        zero_segments.into_iter().fold((0, Option::<usize>::None), |(res, previous_end), (start, end), | {
            if previous_end.is_none() || start >= previous_end.unwrap() {
                (res + 1, Some(end))
            } else {
                (res, previous_end)
            }
        }).0
    }
}

// @code end
