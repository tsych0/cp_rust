// Created by Ayush Biswas at 2025/07/15 22:05
// https://cses.fi/problemset/task/2183
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        _n: usize,
        a: [usize]
    ) -> usize {
        a.into_iter().sorted().try_fold(1, |possible, ai| {
            if ai <= possible {
                Ok(possible + ai)
            } else {
                Err(possible)
            }
        })
        .unwrap_or_else(|e| e)
    }
}

// @code end
