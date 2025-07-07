// Created by Ayush Biswas at 2025/07/02 20:33
// https://codeforces.com/problemset/problem/69/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        forces: [[isize; 3]; n]
    ) -> Bool {
        (
        forces
            .into_iter()
            .fold((0, 0, 0), |(i, j, k), [fx, fy, fz]| {
                (i + fx, j + fy, k + fz)
            })
            == (0, 0, 0)
            ).into()
    }
}

// @code end
