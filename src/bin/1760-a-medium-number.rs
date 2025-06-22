// Created by Ayush Biswas at 2025/06/06 14:53
// https://codeforces.com/problemset/problem/1760/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    a.into_iter().sorted().skip(1).next().unwrap()
}
// @code end
