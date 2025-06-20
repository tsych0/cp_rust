// Created by Ayush Biswas at 2025/06/08 12:13
// https://codeforces.com/problemset/problem/160/A
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let total: usize = a.iter().sum();
    let mut sum = 0;
    for (i, ai) in a.into_iter().sorted_by(|a, &b| b.cmp(a)).enumerate() {
        if 2 * sum > total {
            return i;
        }
        sum += ai;
    }
    n
}
// @code end
