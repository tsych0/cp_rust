// Created by Ayush Biswas at 2025/05/13 22:27
// https://codeforces.com/problemset/problem/2062/A
use cf_rust::cpio;

// @code begin
use cpio::solve_n;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> u8
where
    R: Read,
{
    let s: Vec<u8> = input.read_line(cpio::parse_binary).unwrap();
    s.into_iter().sum()
}

// @code end
