// Created by Ayush Biswas at 2025/05/14 20:25
// https://codeforces.com/problemset/problem/266/A
use cp_rust::*;

// @code begin
use cpio::solve;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> u8
where
    R: Read,
{
    let _: u8 = input.read_line(cpio::parse).unwrap();
    let stones: String = input.read_line(cpio::parse).unwrap();
    stones
        .chars()
        .fold(('X', 0), |acc, stone| {
            if stone == acc.0 {
                (stone, acc.1 + 1)
            } else {
                (stone, acc.1)
            }
        })
        .1
}
// @code end
