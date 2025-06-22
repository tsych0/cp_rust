// Created by Ayush Biswas at 2025/06/08 12:42
// https://codeforces.com/problemset/problem/2072/B

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
    let s: String = input.read_line(parse).unwrap();
    let a = s.chars().filter(|&c| c == '_').count();
    let b = s.chars().filter(|&c| c == '-').count();
    ((b + 1) / 2) * (b / 2) * a
}
// @code end
