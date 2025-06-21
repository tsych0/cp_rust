// Created by Ayush Biswas at 2025/05/19 22:25
// https://codeforces.com/problemset/problem/1791/A
use cp_rust::*;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let c: String = input.read_line(parse).unwrap();
    "codeforces".contains(&c)
}
// @code end
