// Created by Ayush Biswas at 2025/07/18 14:52
// https://cses.fi/problemset/task/1630
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::ops::AddAssign;

sol! {
    fn solution(n: usize, a: [[isize; 2]; n]) -> isize {
        let (times, deadlines): (Vec<_>, Vec<_>) = a
            .into_iter()
            .map(|[time, deadline]| (time, deadline))
            .unzip();

        let delay: isize = times
            .into_iter()
            .sorted()
            .scan(0, |state, ti| {
                state.add_assign(ti);
                Some(*state)
            })
            .sum();

        deadlines.into_iter().sum::<isize>() - delay
    }
}

// @code end
