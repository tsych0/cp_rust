// Created by Ayush Biswas at 2025/07/18 16:02
// https://cses.fi/problemset/task/1661
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::{collections::HashMap, iter::once, ops::AddAssign};

sol! {
    fn solution([n, x]: [isize; 2], a: [isize]) -> usize {
        let prefix_sum = once(0)
            .chain(a.into_iter().scan(0, |sum, ai| {
                sum.add_assign(ai);
                Some(*sum)
            }))
            .collect_vec();
        prefix_sum.into_iter().scan(HashMap::new(), |seen, ai|  {
            let bj = ai - x;
            let r = *seen.get(&bj).unwrap_or(&0);
            seen.entry(ai).or_insert(0).add_assign(1);
            Some(r)
        })
        .sum()
    }
}

// @code end
