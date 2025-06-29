// Created by Ayush Biswas at 2025/06/29 14:55
// https://codeforces.com/problemset/problem/1848/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, m, k]: [usize; 3],
        [x, y]: [isize; 2],
        friends: [[isize; 2]; k]
    ) -> bool {
        for xc in [-1, 0, 1isize] {
        'l:
            for yc in [-1, 0, 1isize] {
                if xc.abs() + yc.abs() > 1 {
                    continue;
                }

                for &[xi, yi] in &friends {
                    if (x.abs_diff(xi) as isize) < (y - yi) * (xc + yc) {
                        continue 'l;
                    }
                }

                return false;
            }
        }
        true
    }
}

// @code end
