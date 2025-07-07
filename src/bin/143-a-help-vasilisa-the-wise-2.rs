// Created by Ayush Biswas at 2025/07/04 21:28
// https://codeforces.com/problemset/problem/143/A
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        [r1, r2]: [usize; 2],
        [c1, c2]: [usize; 2],
        [d1, d2]: [usize; 2]
    ) -> Option<Lines<Words<usize>>> {
        let b2 = d2 + (r1 - c1);
        if b2 % 2 != 0 {
            return None;
        }
        let b = b2 / 2;
        let a = r1 - b;
        let d = c2 - b;
        let c = d2 - b;

        if c + d != r2 || a + c != c1 || a + d != d1
            || vec![a, b, c, d].into_iter().unique().count() < 4
            || vec![a, b, c, d].into_iter().max().unwrap() > 9
            || vec![a, b, c, d].into_iter().min().unwrap() < 1 {
            return None;
        }

        Some(vec![vec![a, b].into(), vec![c, d].into()].into())
    }
}

// @code end
