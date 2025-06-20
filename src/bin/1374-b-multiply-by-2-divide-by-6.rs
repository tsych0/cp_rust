// Created by Ayush Biswas at 2025/06/18 14:34
// https://codeforces.com/problemset/problem/1374/B
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> isize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let (two_count, rem) = (0..)
        .try_fold(n, |acc, i| {
            if acc % 2 == 0 {
                Ok(acc / 2)
            } else {
                Err((i, acc))
            }
        })
        .unwrap_err();
    let (three_count, rem) = (0..)
        .try_fold(rem, |acc, i| {
            if acc % 3 == 0 {
                Ok(acc / 3)
            } else {
                Err((i, acc))
            }
        })
        .unwrap_err();
    if two_count > three_count || rem != 1 {
        -1
    } else {
        2 * three_count - two_count
    }
}
// @code end
