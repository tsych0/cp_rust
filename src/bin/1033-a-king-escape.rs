// Created by Ayush Biswas at 2025/07/04 14:55
// https://codeforces.com/problemset/problem/1033/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        [ax, ay]: [isize; 2],
        [bx, by]: [isize; 2],
        [cx, cy]: [isize; 2]
    ) -> bool {
        (bx - ax)*(cx - ax) > 0 &&
        (by - ay)*(cy - ay) > 0 &&
        (ax + ay) != (cx + cy) &&
        (ax - ay) != (cx - cy)
    }
}

// @code end
