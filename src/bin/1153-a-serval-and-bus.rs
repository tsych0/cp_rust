// Created by Ayush Biswas at 2025/07/06 14:51
// https://codeforces.com/problemset/problem/1153/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, t]: [usize; 2],
        sd: [[usize; 2]; n as usize]
    ) -> usize {
        sd
            .into_iter()
            .map(|[s, d]| {
                if s >= t {
                    s
                } else {
                    s + d * (t - s).div_ceil(d)
                }
            })
            .zip(1..)
            .min()
            .unwrap()
            .1
    }
}

// @code end
