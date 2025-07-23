// Created by Ayush Biswas at 2025/07/10 12:05
// https://cses.fi/problemset/task/1618
use cp_lib::*;

// @code begin
use cpio::*;
use std::ops::DivAssign;

sol! {
    fn solution(
        n: usize,
    ) -> usize {
        (1..)
            .scan(n, |state, _| {
                state.div_assign(5);
                if *state > 0 {
                    Some(*state)
                } else {
                    None
                }
            })
            .sum()
    }
}

// @code end
