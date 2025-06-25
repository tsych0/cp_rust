// Created by Ayush Biswas at 2025/06/13 10:58
// https://codeforces.com/problemset/problem/1833/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

sol! {
    fn (
        [n, _k]: [usize; 2],
        a: [isize],
        (mut b): [isize]
    ) -> Words<isize> {
        b.sort();
         a.into_iter()
             .zip(0..n)
             .sorted()
             .enumerate()
             .map(|(i, (_, j))| (j, i))
             .sorted()
             .map(|(_, i)| b[i])
             .collect()
    }
}

// @code end
