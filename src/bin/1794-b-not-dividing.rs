// Created by Ayush Biswas at 2025/06/12 15:36
// https://codeforces.com/problemset/problem/1794/B
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

fn solution<R>(input: &mut CPInput<R>) -> Words<usize>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let mut a: Vec<usize> = input.read_line(parse_vec).unwrap();
    for i in 0..n {
        if a[i] == 1 {
            a[i] = 2;
        }
    }
    for i in 1..n {
        while a[i] % a[i - 1] == 0 {
            a[i] += 1
        }
    }
    ListOf(a)
}
// @code end
