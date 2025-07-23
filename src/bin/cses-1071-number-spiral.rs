// Created by Ayush Biswas at 2025/07/09 22:58
// https://cses.fi/problemset/task/1071
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [x, y]: [usize; 2]
    ) -> usize {
        let diagonal = x.max(y);
        let diag_value = 1 + (diagonal - 1) * diagonal;
        if diagonal % 2 == 0 {
            diag_value + x - y
        } else {
            diag_value - x + y
        }
    }
}

// @code end
