// Created by Ayush Biswas at 2025/07/18 15:55
// https://cses.fi/problemset/task/1660
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, x]: [usize; 2],
        a: [usize]
    ) -> usize {
        let mut s = 0;
        let mut back = 0;
        let mut front = 0;
        let mut res = 0;
        while front < n {
            while front < n && s < x {
                s += a[front];
                front += 1;
            }
            while s >= x {
                if s == x {
                    res += 1;
                }
                s -= a[back];
                back += 1;
            }
        }
        res
    }
}

// @code end
