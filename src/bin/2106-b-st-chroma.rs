// Created by Ayush Biswas at 2025/06/07 14:31
// https://codeforces.com/problemset/problem/2106/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Words<usize>
where
    R: Read,
{
    let [n, x]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let mut r: Vec<usize> = (0..n).filter(|&i| i != x).collect();
    if x < n {
        r.push(x);
    }
    ListOf(r)
}
// @code end
