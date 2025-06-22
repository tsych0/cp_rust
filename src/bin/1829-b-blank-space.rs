// Created by Ayush Biswas at 2025/06/06 11:34
// https://codeforces.com/problemset/problem/1829/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
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
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    a.into_iter()
        .group_by(|&ai| ai)
        .filter(|g| g[0] == 0)
        .map(|g| g.len())
        .max()
        .unwrap_or(0)
}
// @code end
