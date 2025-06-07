// Created by Ayush Biswas at 2025/06/07 14:31
// https://codeforces.com/problemset/problem/2106/B
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> ListOf<usize>
where
    R: Read,
{
    let [n, x]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let mut r: Vec<usize> = (0..n).filter(|&i| i != x).collect();
    if x < n {
        r.push(x);
    }
    ListOf::WordsOf(r)
}
// @code end
