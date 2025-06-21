// Created by Ayush Biswas at 2025/06/03 10:27
// https://codeforces.com/problemset/problem/144/A
#![allow(unused)]

use cp_rust::*;

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
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let (_, max_idx) = a
        .iter()
        .zip(0..n)
        .max_by(|(ai, idx), (aj, jdx)| ai.cmp(aj).then(jdx.cmp(idx)))
        .unwrap();
    let (_, min_idx) = a.iter().zip((0..n).rev()).min().unwrap();

    if max_idx > (n - min_idx - 1) {
        max_idx + min_idx - 1
    } else {
        max_idx + min_idx
    }
}
// @code end
