// Created by Ayush Biswas at 2025/06/18 15:58
// https://codeforces.com/problemset/problem/1875/A
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
    let [a, b, n]: [usize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let x: Vec<usize> = input.read_line(parse_vec).unwrap();
    x.into_iter().map(|xi| xi.min(a - 1)).sum::<usize>() + b
}
// @code end
