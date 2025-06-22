// Created by Ayush Biswas at 2025/06/03 15:08
// https://codeforces.com/problemset/problem/443/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let s: String = input.read_line(parse).unwrap();
    s.chars()
        .filter(|x| ![',', ' ', '{', '}'].contains(x))
        .unique()
        .count()
}
// @code end
