// Created by Ayush Biswas at 2025/05/14 10:40
// https://codeforces.com/problemset/problem/282/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        n is usize,
        operations is [String]
    ) -> i32
    {
        operations.into_iter().fold(
            0,
            |acc, op| {
                if op.contains('+') {
                    acc + 1
                } else {
                    acc - 1
                }
            },
        )
    }
}

// @code end
