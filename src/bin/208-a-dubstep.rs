// Created by Ayush Biswas at 2025/06/09 00:00
// https://codeforces.com/problemset/problem/208/A
#![allow(unused)]

use cp_rust::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let s: String = input.read_line(parse).unwrap();
    s.split("WUB")
        .filter(|c| !c.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
// @code end
