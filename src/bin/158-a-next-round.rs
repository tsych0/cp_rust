// Created by Ayush Biswas at 2025/05/14 19:55
// https://codeforces.com/problemset/problem/158/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [_, k]: [u8; 2],
        a: [u8]
    ) -> u8 {
        let mut current_rank = 0;
        let mut prev = 0;
        for ai in a.into_iter() {
            if ai == 0 {
                break;
            }
            if ai == prev {
                current_rank += 1;
            } else {
                if current_rank >= k {
                    break;
                }
                current_rank += 1;
            }
            prev = ai;
        }
        current_rank
    }
}
// @code end
