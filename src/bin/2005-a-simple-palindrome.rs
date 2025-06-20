// Created by Ayush Biswas at 2025/06/12 12:12
// https://codeforces.com/problemset/problem/2005/A
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

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    vowels.into_iter().cycle().take(n).sorted().collect()
}
// @code end
