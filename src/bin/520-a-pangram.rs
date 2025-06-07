// Created by Ayush Biswas at 2025/06/03 20:28
// https://codeforces.com/problemset/problem/520/A
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

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let s: String = input.read_line(parse).unwrap();
    s.to_lowercase().chars().unique().count() == 26
}
// @code end
