// Created by Ayush Biswas at 2025/06/15 12:07
// https://codeforces.com/problemset/problem/1665/B
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::cmp::min;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<isize> = input.read_line(parse_vec).unwrap();
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
// @code end
