// Created by Ayush Biswas at 2025/05/20 09:34
// https://codeforces.com/problemset/problem/677/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let [n, h]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    n + a.into_iter().filter(|&ai| ai > h).count()
}
// @code end
