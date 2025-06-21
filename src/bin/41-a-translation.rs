// Created by Ayush Biswas at 2025/05/17 16:39
// https://codeforces.com/problemset/problem/41/A
use cp_rust::*;

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
    let s: String = input.read_line(parse).unwrap();
    let t: String = input.read_line(parse).unwrap();
    s.chars().rev().collect::<String>() == t
}
// @code end
