// Created by Ayush Biswas at 2025/05/14 10:35
// https://codeforces.com/problemset/problem/2084/A
use cp_rust::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(io: &mut cpio::CPInput<R>) -> Words<i16>
where
    R: Read,
{
    let n: i16 = io.read_line(cpio::parse).unwrap();
    if n % 2 == 0 {
        ListOf(vec![-1])
    } else {
        (n..=n).chain(1..n).collect()
    }
}
// @code end
