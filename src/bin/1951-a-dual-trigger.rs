// Created by Ayush Biswas at 2025/06/12 15:49
// https://codeforces.com/problemset/problem/1951/A
#![allow(unused)]

use cp_rust::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::collections::VecDeque;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> bool
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let s: Vec<u8> = input.read_line(parse_binary).unwrap();
    let one_count = s.iter().filter(|&&i| i == 1).count();
    if one_count % 2 == 0 && one_count != 2 {
        true
    } else if one_count == 2 {
        s.into_iter().group_by(|&c| c).filter(|g| g[0] == 1).count() == 2
    } else {
        false
    }
}
// @code end
