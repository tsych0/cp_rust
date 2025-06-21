// Created by Ayush Biswas at 2025/06/08 12:42
// https://codeforces.com/problemset/problem/2072/B
#![allow(unused)]

use cp_rust::*;

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
    let n: usize = input.read_line(parse).unwrap();
    let s: String = input.read_line(parse).unwrap();
    let a = s.chars().filter(|&c| c == '_').count();
    let b = s.chars().filter(|&c| c == '-').count();
    ((b + 1) / 2) * (b / 2) * a
}
// @code end
