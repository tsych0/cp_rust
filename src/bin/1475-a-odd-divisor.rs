// Created by Ayush Biswas at 2025/06/10 16:41
// https://codeforces.com/problemset/problem/1475/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
       (mut n): usize
    ) -> bool {
        while n % 2 == 0 {
            n /= 2
        }
        n != 1
    }
}

// @code end
