// Created by Ayush Biswas at 2025/07/09 22:16
// https://cses.fi/problemset/task/1069
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        a: [char]
    ) -> usize {
        a
            .into_iter()
            .group_by(|&ai| ai)
            .map(|g| g.len())
            .max()
            .unwrap()
    }
}

// @code end
