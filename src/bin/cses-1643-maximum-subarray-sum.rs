// Created by Ayush Biswas at 2025/07/15 21:16
// https://cses.fi/problemset/task/1643
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        _n: usize,
        a: [isize]
    ) -> isize {
        let m = a.iter().max().unwrap().clone();
        a.into_iter().fold((m, 0), |(res, sum), ai| {
            let new_sum = sum + ai;
            if new_sum < 0 {
                (res, 0)
            } else {
                (res.max(new_sum), new_sum)
            }
        }).0
    }
}

// @code end
