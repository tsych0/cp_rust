// Created by Ayush Biswas at 2025/06/03 15:08
// https://codeforces.com/problemset/problem/443/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn solution(
        s: String
    ) -> usize {
        s.chars()
               .filter(|x| ![',', ' ', '{', '}'].contains(x))
               .unique()
               .count()
    }
}

// @code end
