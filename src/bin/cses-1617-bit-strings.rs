// Created by Ayush Biswas at 2025/07/10 12:01
// https://cses.fi/problemset/task/1617
use cp_lib::*;

// @code begin
use cpio::*;
use math::pow_mod;

sol! {
    fn solution(
        n: i64,
    ) -> u32 {
        pow_mod(2, n, 1e9 as u32 +7)
    }
}

// @code end
