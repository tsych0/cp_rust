// Created by Ayush Biswas at 2025/06/07 12:43
// https://codeforces.com/problemset/problem/1878/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<usize>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    (1..=n).map(|i| 2 * i - 1).collect()
}
// @code end
