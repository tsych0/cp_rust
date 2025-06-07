// Created by Ayush Biswas at 2025/06/07 22:28
// https://codeforces.com/problemset/problem/2085/A
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

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let s: String = input.read_line(parse).unwrap();
    if s < s.chars().rev().collect() {
        true
    } else {
        s.chars().unique().count() > 1 && k > 0
    }
}
// @code end
