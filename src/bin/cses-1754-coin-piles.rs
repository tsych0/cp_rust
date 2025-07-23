// Created by Ayush Biswas at 2025/07/10 12:14
// https://cses.fi/problemset/task/1754
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [a, b]: [usize; 2]
    ) -> Bool {
        let [a, b] = [a.min(b), a.max(b)];
        let diff = b - a;

        (a >= diff && (a - diff)%3 == 0).into()
    }
}

// @code end
