// Created by Ayush Biswas at 2025/05/14 17:56
// https://codeforces.com/problemset/problem/2059/A
use cp_lib::*;

// @code begin
use cpio::solve_n;
use std::{collections::HashSet, io::Read};

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> bool
where
    R: Read,
{
    let _n: usize = input.read_line(cpio::parse).unwrap();
    let a: Vec<usize> = input.read_line(cpio::parse_vec).unwrap();
    let b: Vec<usize> = input.read_line(cpio::parse_vec).unwrap();
    a.into_iter().collect::<HashSet<_>>().len() + b.into_iter().collect::<HashSet<_>>().len() > 3
}
// @code end
