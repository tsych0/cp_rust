// Created by Ayush Biswas at 2025/05/20 11:09
// https://codeforces.com/problemset/problem/1978/A
use cf_rust::cpio;

// @code begin
use cpio::*;
#[allow(unused)]
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
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    a.get(0..n - 1).unwrap().iter().max().unwrap() + a.last().unwrap()
}
// @code end
