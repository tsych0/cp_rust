// Created by Ayush Biswas at 2025/06/07 22:28
// https://codeforces.com/problemset/problem/2085/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn solution(
        [_, k]: [usize; 2],
        s: String
    ) -> bool {
        if s < s.chars().rev().collect() {
            true
        } else {
            s.chars().unique().count() > 1 && k > 0
        }
    }
}
// @code end
