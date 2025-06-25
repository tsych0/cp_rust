// Created by Ayush Biswas at 2025/06/15 12:07
// https://codeforces.com/problemset/problem/1665/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::cmp::min;

sol! {
    fn solution(
        n: usize,
        a: [isize]
    ) -> usize {
        let mut max_occ = a
            .into_iter()
            .sorted()
            .group_by(|&x| x)
            .map(|g| g.len())
            .max()
            .unwrap();
        let mut res = 0;
        while max_occ < n {
            res += min(max_occ, n - max_occ);
            max_occ *= 2;
            res += 1;
        }
        res
    }
}

// @code end
