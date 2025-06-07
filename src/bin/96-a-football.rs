// Created by Ayush Biswas at 2025/06/07 14:37
// https://codeforces.com/problemset/problem/96/A
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

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let n: Vec<u8> = input.read_line(parse_binary).unwrap();
    n.into_iter()
        .group_by(|&p| p)
        .map(|g| g.len())
        .max()
        .unwrap()
        >= 7
}
// @code end
