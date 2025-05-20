// Created by Ayush Biswas at 2025/05/20 08:50
// https://codeforces.com/problemset/problem/228/A
use cf_rust::cpio;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::{collections::HashSet, io::Read};

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let shoes: Vec<usize> = input.read_line(parse_vec).unwrap();
    4 - shoes.into_iter().collect::<HashSet<_>>().len()
}
// @code end
