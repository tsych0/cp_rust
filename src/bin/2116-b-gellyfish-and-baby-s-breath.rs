// Created by Ayush Biswas at 2025/06/02 18:51
// https://codeforces.com/problemset/problem/2116/B
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

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    n > 2 && n % 2 == 0
}
// @code end
