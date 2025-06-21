// Created by Ayush Biswas at 2025/06/03 21:30
// https://codeforces.com/problemset/problem/1791/C
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
    let n: usize = input.read_line(parse).unwrap();
    let s: Vec<u8> = input.read_line(parse_binary).unwrap();
    let (mut i, mut j) = (0, n - 1);
    while i < j {
        if s[i] + s[j] == 1 {
            i += 1;
            j -= 1;
        } else {
            break;
        }
    }
    j - i + 1
}
// @code end
