// Created by Ayush Biswas at 2025/07/14 15:11
// https://cses.fi/problemset/task/1621
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        a.into_iter().unique().count()
    }
}

// @code end
