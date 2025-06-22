// Created by Ayush Biswas at 2025/05/14 17:47
// https://codeforces.com/problemset/problem/112/A
use cp_lib::*;

// @code begin
use cpio::solve;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut cpio::CPInput<R>) -> i8
where
    R: Read,
{
    let a: String = input.read_line(cpio::parse).unwrap();
    let b: String = input.read_line(cpio::parse).unwrap();

    use std::cmp::Ordering::*;
    match a.to_lowercase().cmp(&b.to_lowercase()) {
        Greater => 1,
        Equal => 0,
        Less => -1,
    }
}
// @code end
