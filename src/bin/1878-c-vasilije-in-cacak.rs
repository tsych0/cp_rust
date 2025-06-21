// Created by Ayush Biswas at 2025/06/16 11:26
// https://codeforces.com/problemset/problem/1878/C
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

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let [n, k, x]: [usize; 3] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let l = n - k;
    let full_sum = (n * (n + 1)) / 2;
    let half_sum = (k * (k + 1)) / 2;
    let quater_sum = (l * (l + 1)) / 2;

    x >= half_sum && x <= (full_sum - quater_sum)
}
// @code end
