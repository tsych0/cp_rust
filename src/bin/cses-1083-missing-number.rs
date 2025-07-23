// Created by Ayush Biswas at 2025/07/09 22:11
// https://cses.fi/problemset/task/1083
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        let observed = a.into_iter().fold(0, |acc, ai| acc ^ ai);
        let theoretical = (1..=n).fold(0, |acc, i| acc ^ i);
        observed ^ theoretical
    }
}

// @code end
