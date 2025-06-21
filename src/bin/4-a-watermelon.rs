// Created by Ayush Biswas at 2025/05/13 23:06
// https://codeforces.com/problemset/problem/4/A
use cp_rust::*;

// @code begin
use cpio::solve;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(io: &mut cpio::CPInput<R>) -> bool
where
    R: Read,
{
    let n: usize = io.read_line(cpio::parse).unwrap();
    n > 2 && n % 2 == 0
}
// @code end
