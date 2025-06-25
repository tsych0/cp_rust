// Created by Ayush Biswas at 2025/05/15 13:42
// https://codeforces.com/problemset/problem/2031/A
use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn (
        n: usize,
        h: [u8]
    ) -> usize {
        n - h
              .into_iter()
              .group_by(|&n| n)
              .into_iter()
              .map(|grp| grp.len())
              .max()
              .unwrap()
    }
}

// @code end
