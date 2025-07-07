// Created by Ayush Biswas at 2025/07/06 01:10
// https://codeforces.com/problemset/problem/552/a
use cp_lib::*;

// @code begin
use cpio::*;
use std::ops::Add;

sol! {
    fn solution(
        n: usize,
        rectangles: [[usize; 4]; n]
    ) -> usize {
        rectangles
            .into_iter()
            .map(|[x1, y1, x2, y2]|
                    x2.abs_diff(x1).add(1) * y2.abs_diff(y1).add(1))
            .sum()
    }
}

// @code end
