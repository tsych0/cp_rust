// Created by Ayush Biswas at 2025/05/18 20:24
// https://codeforces.com/problemset/problem/116/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        abs: [[usize]]; n
    ) -> usize {
        abs.into_iter()
            .fold((0, 0), |(curr, cap), ab| {
                let curr = curr - ab[0] + ab[1];
                let cap = cap.max(curr);
                (curr, cap)
            })
            .1
    }
}

// @code end
