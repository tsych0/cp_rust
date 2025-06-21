// Created by Ayush Biswas at 2025/05/14 10:42
// https://codeforces.com/problemset/problem/2065/A
use cp_rust::*;

// @code begin
use cpio::solve_n;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> String
where
    R: Read,
{
    let mut s: String = input.read_line(cpio::parse).unwrap();
    let len = s.len();
    s.replace_range(len - 2.., "i");
    s
}
// @code end
