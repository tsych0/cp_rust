// Created by Ayush Biswas at 2025/05/14 10:45
// https://codeforces.com/problemset/problem/231/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        confidence: [[u16]; n]
    ) -> u16 {
        confidence
            .into_iter()
            .map(|v| v.into_iter().sum::<u16>() / 2)
            .sum()
    }
}

// @code end
