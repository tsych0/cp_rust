// Created by Ayush Biswas at 2025/05/20 09:23
// https://codeforces.com/problemset/problem/200/B
use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> f64
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    a.into_iter().sum::<usize>() as f64 / n as f64
}
// @code end
