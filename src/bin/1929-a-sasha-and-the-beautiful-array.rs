// Created by Ayush Biswas at 2025/06/03 15:17
// https://codeforces.com/problemset/problem/1929/A

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
    let _n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    a.iter().max().unwrap() - a.iter().min().unwrap()
}
// @code end
