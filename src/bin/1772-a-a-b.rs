// Created by Ayush Biswas at 2025/06/04 10:54
// https://codeforces.com/problemset/problem/1772/A
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
    s.split('+').map(|x| x.parse::<usize>().unwrap()).sum()
}
// @code end
