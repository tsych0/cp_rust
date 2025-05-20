// Created by Ayush Biswas at 2025/05/16 12:21
// https://codeforces.com/problemset/problem/2030/B
use cf_rust::cpio;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();

    vec!['0'; n - 1].into_iter().collect::<String>() + "1"
}
// @code end
