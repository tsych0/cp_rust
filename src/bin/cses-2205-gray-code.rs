// Created by Ayush Biswas at 2025/07/10 12:58
// https://cses.fi/problemset/task/2205
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
    ) -> Lines<ListOf<'\0', u8>> {
        (0..1<<n).map(|i| gray(i, n).into()).collect()
    }
}

fn gray(i: usize, n: usize) -> Vec<u8> {
    (0..n)
        .rev()
        .map(|j| {
            let a = (i >> (j + 1)) % 2;
            let b = (i >> j) % 2;
            (a ^ b) as u8
        })
        .collect()
}

// @code end
