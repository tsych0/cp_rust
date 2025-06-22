// Created by Ayush Biswas at 2025/06/10 22:14
// https://codeforces.com/problemset/problem/1837/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let s: String = input.read_line(parse).unwrap();
    s.chars().group_by(|&c| c).map(|g| g.len()).max().unwrap() + n % 2
}
// @code end
