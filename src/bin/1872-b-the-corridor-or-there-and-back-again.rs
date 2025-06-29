// Created by Ayush Biswas at 2025/06/26 22:59
// https://codeforces.com/problemset/problem/1872/B
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol_n! {
    fn solution(
        n: usize,
        ds: [[usize; 2]; n]
    ) -> usize {
        let res = ds.into_iter().sorted().try_fold((0, usize::MAX), |(curr_pos, life_line), [di, seconds]| {
            if life_line == 0 {
                Err((curr_pos, life_line))
            } else {
                if life_line >= (di - curr_pos) {
                    Ok((di, usize::min(life_line - (di - curr_pos), (seconds+1)/2 - 1)))
                } else {
                    Err((curr_pos, life_line))
                }
            }
        });

        match res {
            Ok((d, s)) => d + s,
            Err((d, s)) => d + s
        }
    }
}

// @code end
