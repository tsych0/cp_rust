// Created by Ayush Biswas at 2025/06/03 15:08
// https://codeforces.com/problemset/problem/443/A
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
    let s: String = input.read_line(parse).unwrap();
    s.chars()
        .filter(|x| ![',', ' ', '{', '}'].contains(x))
        .unique()
        .count()
}
// @code end
