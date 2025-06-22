// Created by Ayush Biswas at 2025/05/16 17:21
// https://codeforces.com/problemset/problem/2030/A
use cp_lib::*;

// @code begin
use cpio::*;
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
    let b = a.iter().min().unwrap();
    let c = a.iter().max().unwrap();
    (c - b) * (n - 1)
}
// @code end
