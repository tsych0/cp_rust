// Created by Ayush Biswas at 2025/05/14 10:41
// https://codeforces.com/problemset/problem/339/A
use cp_rust::*;

// @code begin
use cpio::solve;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> String
where
    R: Read,
{
    let addition: String = input.read_line(cpio::parse).unwrap();
    let mut nums = addition.split('+').collect::<Vec<_>>();
    nums.sort();
    nums.join("+")
}
// @code end
