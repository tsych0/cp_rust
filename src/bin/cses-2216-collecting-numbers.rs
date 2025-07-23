// Created by Ayush Biswas at 2025/07/17 11:27
// https://cses.fi/problemset/task/2216
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        a.into_iter().scan(vec![false; n + 1], |state, ai| {
            state[ai - 1] = true;
            Some(state[ai].then(|| 1).unwrap_or(0))
        })
        .sum::<usize>() + 1
    }
}

// @code end
