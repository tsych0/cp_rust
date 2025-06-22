// Created by Ayush Biswas at 2025/06/13 10:58
// https://codeforces.com/problemset/problem/1833/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<isize>
where
    R: Read,
{
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let a: Vec<isize> = input.read_line(parse_vec).unwrap();
    let mut b: Vec<isize> = input.read_line(parse_vec).unwrap();
    b.sort();
    a.into_iter()
        .zip(0..n)
        .sorted()
        .enumerate()
        .map(|(i, (_, j))| (j, i))
        .sorted()
        .map(|(j, i)| b[i])
        .collect()
}
// @code end
