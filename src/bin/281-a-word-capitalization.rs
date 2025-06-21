// Created by Ayush Biswas at 2025/05/14 18:01
// https://codeforces.com/problemset/problem/281/A
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
    let s: String = input.read_line(cpio::parse).unwrap();
    let mut c = s.chars();
    format!(
        "{}{}",
        c.next().unwrap().to_uppercase(),
        c.collect::<String>()
    )
}
// @code end
