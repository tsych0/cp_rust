// Created by Ayush Biswas at 2025/05/15 12:33
// https://codeforces.com/problemset/problem/59/A
use cp_rust::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let s: String = input.read_line(parse).unwrap();
    if s.chars().into_iter().filter(|c| c.is_lowercase()).count()
        < s.chars().into_iter().filter(|c| c.is_uppercase()).count()
    {
        s.to_uppercase()
    } else {
        s.to_lowercase()
    }
}
// @code end
