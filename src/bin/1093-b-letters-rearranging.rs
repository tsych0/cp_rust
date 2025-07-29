// Created by Ayush Biswas at 2025/06/28 19:32
// https://codeforces.com/problemset/problem/1093/B
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol_n! {
    fn solution(
        s: String
    ) -> CPResult<String, i8> {
        if s.chars().unique().count() > 1 {
            Success(s.chars().sorted().collect::<String>())
        } else {
            Failure(-1)
        }
    }
}

// @code end
