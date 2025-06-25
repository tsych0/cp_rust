// Created by Ayush Biswas at 2025/06/03 10:12
// https://codeforces.com/problemset/problem/1935/A

use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
        s: String
    ) -> String {
        let s_rev: String = s.chars().rev().collect();

        if s > s_rev {
            if n % 2 == 0 {
                format!("{s_rev}{s}")
            } else {
                s_rev
            }
        } else {
            if n % 2 == 0 {
                s
            } else {
                format!("{s}{s_rev}")
            }
        }
    }
}

// @code end
