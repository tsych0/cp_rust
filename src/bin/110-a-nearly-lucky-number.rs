// Created by Ayush Biswas at 2025/05/16 12:05
// https://codeforces.com/problemset/problem/110/A
use cf_rust::cpio;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let n: String = input.read_line(parse).unwrap();
    n.chars()
        .filter(|&c| c == '4' || c == '7')
        .count()
        .to_string()
        .chars()
        .all(|c| c == '4' || c == '7')
}
// @code end
