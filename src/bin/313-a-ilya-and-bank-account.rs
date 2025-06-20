// Created by Ayush Biswas at 2025/06/13 15:15
// https://codeforces.com/problemset/problem/313/A
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::cmp::max;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> isize
where
    R: Read,
{
    let n: isize = input.read_line(parse).unwrap();
    if n >= 0 {
        n
    } else {
        let l = n % 10;
        max((n / 100) * 10 + l, n / 10)
    }
}
// @code end
