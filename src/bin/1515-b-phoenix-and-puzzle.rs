// Created by Ayush Biswas at 2025/07/04 12:36
// https://codeforces.com/problemset/problem/1515/B
use cp_lib::*;

// @code begin
use cpio::*;
use pathfinding::uint_sqrt;

sol_n! {
    fn solution(
        n: u32,
    ) -> bool {
        (n%4 == 0 && uint_sqrt(n/4).is_some())
            || (n%2 == 0 && uint_sqrt(n/2).is_some())
    }
}

// @code end
