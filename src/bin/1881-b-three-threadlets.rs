// Created by Ayush Biswas at 2025/06/21 13:56
// https://codeforces.com/problemset/problem/1881/B

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize
    ) -> bool {
        n > 2 && n % 2 == 0
    }
}
// @code end
