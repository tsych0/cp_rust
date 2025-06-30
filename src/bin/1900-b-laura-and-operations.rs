// Created by Ayush Biswas at 2025/06/30 16:11
// https://codeforces.com/problemset/problem/1900/B
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [a, b, c]: [usize; 3]
    ) -> Words<usize> {
        vec![(b+c+1)%2, (a+c+1)%2, (a+b+1)%2].into()
    }
}

// @code end
