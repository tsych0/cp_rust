// Created by Ayush Biswas at 2025/06/04 10:32
// https://codeforces.com/problemset/problem/1800/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
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
    let s: String = input.read_line(parse).unwrap();
    s.to_lowercase()
        .chars()
        .group_by(|&c| c)
        .map(|g| g[0])
        .collect::<String>()
        == "meow"
}
// @code end
