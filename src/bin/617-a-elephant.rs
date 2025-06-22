// Created by Ayush Biswas at 2025/05/17 10:30
// https://codeforces.com/problemset/problem/617/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let x: usize = input.read_line(parse).unwrap();
    if x % 5 == 0 {
        x / 5
    } else {
        x / 5 + 1
    }
}
// @code end
