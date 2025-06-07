// Created by Ayush Biswas at 2025/06/03 20:14
// https://codeforces.com/problemset/problem/1917/A
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

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<isize> = input.read_line(parse_vec).unwrap();
    if a.iter().filter(|&&ai| ai < 0).count() % 2 == 0
        && a.iter().filter(|&&ai| ai == 0).count() == 0
    {
        "1\n1 0".into()
    } else {
        "0".into()
    }
}
// @code end
