// Created by Ayush Biswas at 2025/06/19 11:37
// https://codeforces.com/problemset/problem/1467/A
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

fn solution<R>(input: &mut CPInput<R>) -> ListOf<'\0', usize>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let shift = 10 - n % 10;
    vec![9]
        .into_iter()
        .chain((1..n).map(|i| (i + 7) % 10))
        .collect()
}
// @code end
