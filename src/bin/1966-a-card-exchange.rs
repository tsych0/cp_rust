// Created by Ayush Biswas at 2025/05/20 11:30
// https://codeforces.com/problemset/problem/1966/A
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
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let mut c: Vec<usize> = input.read_line(parse_vec).unwrap();
    c.sort();
    let max_group_size = c
        .into_iter()
        .group_by(|&ci| ci)
        .map(|g| g.len())
        .max()
        .unwrap();
    if max_group_size >= k {
        k - 1
    } else {
        n
    }
}
// @code end
