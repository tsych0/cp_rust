// Created by Ayush Biswas at 2025/05/17 16:12
// https://codeforces.com/problemset/problem/2008/A
use cf_rust::cpio;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let [a, b]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    a % 2 == 0 && (b % 2 == 0 || a >= 2)
}
// @code end
