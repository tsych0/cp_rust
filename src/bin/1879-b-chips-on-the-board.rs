// Created by Ayush Biswas at 2025/06/12 23:42
// https://codeforces.com/problemset/problem/1879/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::cmp::min;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> usize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<usize> = input.read_line(parse_vec).unwrap();
    let b: Vec<usize> = input.read_line(parse_vec).unwrap();
    let (a_sum, a_min) = (a.iter().sum::<usize>(), a.iter().min().unwrap());
    let (b_sum, b_min) = (b.iter().sum::<usize>(), b.iter().min().unwrap());
    min(a_sum + b_min * n, b_sum + a_min * n)
}
// @code end
