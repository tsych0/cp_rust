// Created by Ayush Biswas at 2025/06/15 12:28
// https://codeforces.com/problemset/problem/1617/B
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
    let res = if n % 2 == 0 {
        vec![n / 2 - 1, n / 2, 1]
    } else {
        if n % 4 == 1 {
            vec![n / 2 - 1, n / 2 + 1, 1]
        } else {
            vec![n / 2 - 2, n / 2 + 2, 1]
        }
    };

    ListOf(res)
}
// @code end
