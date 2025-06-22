// Created by Ayush Biswas at 2025/06/13 12:07
// https://codeforces.com/problemset/problem/1806/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> u8
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    if a.iter().filter(|&&i| i == 0).count() > (n + 1) / 2 {
        if a.into_iter().max().unwrap() == 1 {
            2
        } else {
            1
        }
    } else {
        0
    }
}
// @code end
