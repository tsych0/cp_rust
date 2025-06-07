// Created by Ayush Biswas at 2025/06/03 16:22
// https://codeforces.com/problemset/problem/1703/A
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

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
    let s: String = input.read_line(parse).unwrap();
    s.to_uppercase() == "YES"
}
// @code end
