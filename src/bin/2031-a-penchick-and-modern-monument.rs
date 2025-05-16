// Created by Ayush Biswas at 2025/05/15 13:42
// https://codeforces.com/problemset/problem/2031/A
use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let h: Vec<u8> = input.read_line(parse_vec).unwrap();
    n - h
        .into_iter()
        .group_by(|&n| n)
        .into_iter()
        .map(|grp| grp.len())
        .max()
        .unwrap()
}
// @code end
