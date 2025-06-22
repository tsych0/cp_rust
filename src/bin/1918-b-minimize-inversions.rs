// Created by Ayush Biswas at 2025/06/12 23:25
// https://codeforces.com/problemset/problem/1918/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Lines<Words<usize>>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let b: Vec<usize> = input.read_line(parse_vec).unwrap();

    let (a, b): (Vec<_>, Vec<_>) = a.into_iter().zip(b.into_iter()).sorted().unzip();
    vec![a.into(), b.into()].into()
}
// @code end
