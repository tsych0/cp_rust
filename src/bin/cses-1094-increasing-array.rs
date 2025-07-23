// Created by Ayush Biswas at 2025/07/09 22:20
// https://cses.fi/problemset/task/1094
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        a
            .into_iter()
            .scan(0, |max_seen, ai| {
                *max_seen = (*max_seen).max(ai);
                Some(*max_seen - ai)
            })
            .sum()
    }
}

// @code end
