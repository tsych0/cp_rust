// Created by Ayush Biswas at 2025/06/08 12:51
// https://codeforces.com/problemset/problem/133/A
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
    let s: String = input.read_line(parse).unwrap();
    s.contains(&['H', 'Q', '9'])
}
// @code end
