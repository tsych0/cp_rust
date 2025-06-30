// Created by Ayush Biswas at 2025/06/29 14:55
// https://codeforces.com/problemset/problem/1848/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [_n, _m, k]: [usize; 3],
        [x, y]: [isize; 2],
        friends: [[isize; 2]; k]
    ) -> Bool {
        let color = (x + y) % 2;
        for [xi, yi] in friends {
            if (xi + yi) % 2 == color {
                return false.into();
            }
        }
        true.into()
    }
}

// @code end
