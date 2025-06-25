// Created by Ayush Biswas at 2025/05/17 10:14
// https://codeforces.com/problemset/problem/2022/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [_n, r]: [usize; 2],
        a: [usize]
    ) -> usize {
        let rows_used: usize = a.iter().map(|ai| ai / 2).sum();
        let people_left: usize = a.iter().map(|ai| ai % 2).sum();
        let rows_left = r - rows_used;
        if people_left <= rows_left {
            rows_used * 2 + people_left
        } else {
            rows_used * 2 + rows_left - (people_left - rows_left)
        }
    }
}

// @code end
