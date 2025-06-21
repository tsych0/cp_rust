// Created by Ayush Biswas at 2025/05/20 11:20
// https://codeforces.com/problemset/problem/1969/A
use cp_rust::*;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> u8
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let p: Vec<usize> = input.read_line(parse_vec).unwrap();

    for i in 0..n {
        if i == p[p[i] - 1] - 1 {
            return 2;
        }
    }

    return 3;
}
// @code end
