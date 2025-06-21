// Created by Ayush Biswas at 2025/06/03 16:33
// https://codeforces.com/problemset/problem/1328/A
#![allow(unused)]

use cp_rust::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let [a, b]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    if a % b == 0 {
        0
    } else {
        b - a % b
    }
}
// @code end
