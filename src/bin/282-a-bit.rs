// Created by Ayush Biswas at 2025/05/14 10:40
// https://codeforces.com/problemset/problem/282/A
use cp_lib::*;

// @code begin
use cpio::solve;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> i32
where
    R: Read,
{
    let n: usize = input.read_line(cpio::parse).unwrap();
    let operations: Vec<String> = input.read_lines(n, cpio::parse).unwrap();
    operations.into_iter().fold(
        0,
        |acc, op| {
            if op.contains('+') {
                acc + 1
            } else {
                acc - 1
            }
        },
    )
}
// @code end
