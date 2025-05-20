// Created by Ayush Biswas at 2025/05/17 15:49
// https://codeforces.com/problemset/problem/271/A
use cf_rust::cpio;

// @code begin
use cpio::*;
use std::{collections::HashSet, io::Read};

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    (n + 1..)
        .find(|&m| m.to_string().chars().collect::<HashSet<_>>().len() == 4)
        .unwrap()
}
// @code end
