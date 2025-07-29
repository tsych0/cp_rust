// Created by Ayush Biswas at 2025/06/07 13:19
// https://codeforces.com/problemset/problem/1870/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, k, x]: [usize; 3]
    ) -> CPResult<usize, i8> {
        if k > n || k > x + 1 {
            return Err(-1).into();
        }
        Ok(if x == k {
            (0..k).sum::<usize>() + (n - k) * (x - 1)
        } else {
            (0..k).sum::<usize>() + (n - k) * x
        }).into()
    }
}

// @code end
