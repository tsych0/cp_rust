// Created by Ayush Biswas at 2025/06/13 12:41
// https://codeforces.com/problemset/problem/1475/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
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
    n / 2020 >= n % 2020
}
// @code end
