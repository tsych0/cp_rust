// Created by Ayush Biswas at 2025/06/06 14:53
// https://codeforces.com/problemset/problem/1760/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol_n! {
    fn solution(
        a: [usize]
    ) -> usize {
        a.into_iter().sorted().skip(1).next().unwrap()
    }
}

// @code end
