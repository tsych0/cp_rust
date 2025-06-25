// Created by Ayush Biswas at 2025/05/20 11:30
// https://codeforces.com/problemset/problem/1966/A
use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    fn (
        [n, k]: [usize; 2],
        (mut c): [usize]
    ) -> usize {
        c.sort();
        let max_group_size = c
            .into_iter()
            .group_by(|&ci| ci)
            .map(|g| g.len())
            .max()
            .unwrap();
        if max_group_size >= k {
            k - 1
        } else {
            n
        }
    }
}

// @code end
