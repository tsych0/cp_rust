// Created by Ayush Biswas at 2025/06/06 15:27
// https://codeforces.com/problemset/problem/996/A
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let denominations = [100, 20, 10, 5, 1];
    denominations
        .into_iter()
        .fold((0, n), |(res, rem), d| (res + rem / d, rem % d))
        .0
}
// @code end
