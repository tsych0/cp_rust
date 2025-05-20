// Created by Ayush Biswas at 2025/05/18 20:24
// https://codeforces.com/problemset/problem/116/A
use cf_rust::cpio;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let abs: Vec<Vec<usize>> = input.read_lines(n, parse_vec).unwrap();
    abs.into_iter()
        .fold((0, 0), |(curr, cap), ab| {
            let curr = curr - ab[0] + ab[1];
            let cap = cap.max(curr);
            (curr, cap)
        })
        .1
}
// @code end
