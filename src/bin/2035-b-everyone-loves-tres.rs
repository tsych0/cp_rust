// Created by Ayush Biswas at 2025/06/09 12:33
// https://codeforces.com/problemset/problem/2035/B
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::Read;
use std::iter::repeat;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    if n % 2 == 0 {
        repeat('3').take(n - 2).collect::<String>() + "66"
    } else if n >= 5 {
        repeat('3').take(n - 5).collect::<String>() + "36366"
    } else {
        "-1".into()
    }
}
// @code end
