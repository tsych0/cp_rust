// Created by Ayush Biswas at 2025/06/12 11:55
// https://codeforces.com/problemset/problem/1850/D
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
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let mut a: Vec<usize> = input.read_line(parse_vec).unwrap();
    a.sort();
    let mut r = vec![0; n];
    for i in 1..n {
        if a[i] - a[i - 1] > k {
            r[i] = r[i - 1] + 1;
        } else {
            r[i] = r[i - 1];
        }
    }
    n - r
        .into_iter()
        .group_by(|&i| i)
        .map(|g| g.len())
        .max()
        .unwrap()
}
// @code end
