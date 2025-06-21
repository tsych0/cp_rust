// Created by Ayush Biswas at 2025/06/13 11:59
// https://codeforces.com/problemset/problem/1856/B
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
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let one_count = a.iter().filter(|&&i| i == 1).count();
    let other_sum = a.iter().filter(|&&i| i != 1).sum::<usize>();
    let remaining_idx = n - one_count;

    other_sum - remaining_idx + one_count >= 2 * one_count && n > 1
}
// @code end
