// Created by Ayush Biswas at 2025/05/14 10:45
// https://codeforces.com/problemset/problem/231/A
use cp_lib::*;

// @code begin
use cpio::solve;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(io: &mut cpio::CPInput<R>) -> u16
where
    R: Read,
{
    let n: usize = io.read_line(cpio::parse).unwrap();
    let confidence: Vec<Vec<u16>> = io.read_lines(n, cpio::parse_vec).unwrap();
    confidence
        .into_iter()
        .map(|v| v.into_iter().sum::<u16>() / 2)
        .sum()
}
// @code end
