// Created by Ayush Biswas at 2025/06/05 16:47
// https://codeforces.com/problemset/problem/1900/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let s: Vec<char> = input.read_line(parse_chars).unwrap();
    let dry_areas = s
        .into_iter()
        .group_by(|&c| c)
        .filter(|g| g[0] == '.')
        .map(|g| g.len())
        .collect::<Vec<_>>();

    if *dry_areas.iter().max().unwrap_or(&0) < 3 {
        dry_areas.iter().sum()
    } else {
        2
    }
}
// @code end
