// Created by Ayush Biswas at 2025/07/10 11:34
// https://cses.fi/problemset/task/1072
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
    ) -> Lines<usize> {
        (1..=n).map(|i| {
            let squares = i * i;
            let combinations = if squares%2 == 0 {
                (squares/2) * (squares - 1)
            } else {
                squares * ((squares - 1) / 2)
            };
            combinations - 4*(i - 1)*(i - 2)
        })
        .collect()
    }
}

// @code end
