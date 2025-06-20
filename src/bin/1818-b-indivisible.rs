// Created by Ayush Biswas at 2025/06/13 11:48
// https://codeforces.com/problemset/problem/1818/B
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

fn solution<R>(input: &mut CPInput<R>) -> Words<isize>
where
    R: Read,
{
    let n: isize = input.read_line(parse).unwrap();
    let res = if n == 1 {
        vec![1]
    } else if n % 2 == 1 {
        vec![-1]
    } else {
        (1..=n / 2)
            .map(|i| vec![2 * i, 2 * i - 1])
            .collect::<Vec<_>>()
            .concat()
    };

    ListOf(res)
}
// @code end
