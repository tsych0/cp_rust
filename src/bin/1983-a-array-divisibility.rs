// Created by Ayush Biswas at 2025/05/20 09:30
// https://codeforces.com/problemset/problem/1983/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<usize>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    (1..=n).collect()
}
// @code end
