// Created by Ayush Biswas at 2025/06/30 15:16
// https://codeforces.com/problemset/problem/1373/B
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        s: [01]
    ) -> String {
        let n = s.len() as u8;
        let one_count = s.into_iter().sum::<u8>();
        let zero_count = n - one_count;
        let max_moves = one_count.min(zero_count);
        if max_moves % 2 == 0 {
            "NET"
        } else {
            "DA"
        }.into()
    }
}

// @code end
