// Created by Ayush Biswas at 2025/06/03 21:45
// https://codeforces.com/problemset/problem/1914/B
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

fn solution<R>(input: &mut CPInput<R>) -> Words<usize>
where
    R: Read,
{
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let cut_point = n - k;

    [
        (cut_point..=n).collect::<Vec<_>>(),
        (1..cut_point).rev().collect::<Vec<_>>(),
    ]
    .concat()
    .into()
}
// @code end
