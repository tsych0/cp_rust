// Created by Ayush Biswas at 2025/06/03 16:26
// https://codeforces.com/problemset/problem/1925/A
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

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();

    vec![('a'..).take(k).collect::<String>(); n].concat()
}
// @code end
