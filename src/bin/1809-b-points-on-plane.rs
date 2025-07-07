// Created by Ayush Biswas at 2025/07/07 16:49
// https://codeforces.com/problemset/problem/1809/b
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(n: isize) -> isize {
        let mut l = -1 as isize;
        let mut r = 1e9 as isize;
        while r - l > 1 {
            let mid = (l + r) / 2;
            if mid * mid >= n {
                r = mid;
            } else {
                l = mid;
            }
        }
        r - 1
    }
}

// @code end
