// Created by Ayush Biswas at 2025/06/29 20:11
// https://codeforces.com/problemset/problem/556/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        a: [01]
    ) -> usize {
        let ones = a.into_iter().map(|x| x as usize).sum::<usize>();
        let zeroes = n - ones;
        n - 2*ones.min(zeroes)
    }
}

// @code end
