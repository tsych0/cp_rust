// Created by Ayush Biswas at 2025/06/06 15:16
// https://codeforces.com/problemset/problem/1829/A
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

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let s: String = input.read_line(parse).unwrap();
    s.chars()
        .zip("codeforces".to_string().chars())
        .filter(|(a, b)| a != b)
        .count()
}
// @code end
