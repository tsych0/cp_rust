// Created by Ayush Biswas at 2025/05/14 20:54
// https://codeforces.com/problemset/problem/2039/A
use cf_rust::cpio;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<u8>
where
    R: Read,
{
    let n: u8 = input.read_line(parse).unwrap();
    (0..n).map(|i| 2 * i + 1).collect()
}
// @code end
