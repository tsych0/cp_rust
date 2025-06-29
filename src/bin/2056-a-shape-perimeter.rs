// Created by Ayush Biswas at 2025/05/14 20:12
// https://codeforces.com/problemset/problem/2056/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, m]: [u16; 2],
        xys: [[u16]; n as usize]
    ) -> u16 {
        (4 * m)
            + xys
                .into_iter()
                .skip(1)
                .map(|xy| (xy[0] + xy[1]) * 2)
                .sum::<u16>()
    }
}

// @code end
