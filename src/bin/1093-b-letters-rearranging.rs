// Created by Ayush Biswas at 2025/06/28 19:32
// https://codeforces.com/problemset/problem/1093/B
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol_n! {
    fn solution(
        s: String
    ) -> String {
        if s.chars().unique().count() > 1 {
            s.chars().sorted().collect()
        } else {
            "-1".into()
        }
    }
}

// @code end
