// Created by Ayush Biswas at 2025/06/04 10:41
// https://codeforces.com/problemset/problem/1905/A
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::cmp::max;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let [n, m]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    max(n, m)
}
// @code end
