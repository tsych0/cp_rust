// Created by Ayush Biswas at 2025/06/12 23:59
// https://codeforces.com/problemset/problem/1869/A
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

fn solution<R>(input: &mut CPInput<R>) -> Lines<String>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    vec![
        "4".into(),
        format!("1 {n}"),
        format!("{} {n}", 1 + n % 2),
        format!("1 2"),
        format!("1 2"),
    ]
    .into()
}
// @code end
