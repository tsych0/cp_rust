// Created by Ayush Biswas at 2025/05/20 11:12
// https://codeforces.com/problemset/problem/486/A
use cf_rust::cpio;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> isize
where
    R: Read,
{
    let n: isize = input.read_line(parse).unwrap();
    if n % 2 == 0 {
        (n + 1) / 2
    } else {
        -(n + 1) / 2
    }
}
// @code end
