// Created by Ayush Biswas at 2025/06/04 10:50
// https://codeforces.com/problemset/problem/1902/A
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
    let n: usize = input.read_line(parse).unwrap();
    let s: Vec<u8> = input.read_line(parse_binary).unwrap();
    let s = s.into_iter().unique().collect::<Vec<_>>();
    s != vec![1]
}
// @code end
