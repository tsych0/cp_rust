// Created by Ayush Biswas at 2025/05/14 20:25
// https://codeforces.com/problemset/problem/266/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        _: u8,
        stones: String
    ) -> u8 {
        stones
            .chars()
            .fold(('X', 0), |acc, stone| {
                if stone == acc.0 {
                    (stone, acc.1 + 1)
                } else {
                    (stone, acc.1)
                }
            })
            .1
    }
}
// @code end
