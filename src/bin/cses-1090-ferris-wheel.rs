// Created by Ayush Biswas at 2025/07/15 15:20
// https://cses.fi/problemset/task/1090
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, x]: [usize; 2],
        (mut p): [usize]
    ) -> usize {
        p.sort();
        let (mut front, mut back) = (0, n - 1);
        let mut res = 0;
        while front <= back {
            if p[front] + p[back] <= x {
                front += 1;
                back = back.saturating_sub(1);
                res += 1;
            } else {
                res += 1;
                if back == 0 {
                    break;
                } else {
                    back -= 1;
                }
            }
        }
        res
    }
}

// @code end
