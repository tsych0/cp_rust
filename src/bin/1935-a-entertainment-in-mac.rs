// Created by Ayush Biswas at 2025/06/03 10:12
// https://codeforces.com/problemset/problem/1935/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
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
    let s: String = input.read_line(parse).unwrap();

    let s_rev: String = s.chars().rev().collect();

    if s > s_rev {
        if n % 2 == 0 {
            format!("{s_rev}{s}")
        } else {
            s_rev
        }
    } else {
        if n % 2 == 0 {
            s
        } else {
            format!("{s}{s_rev}")
        }
    }
}
// @code end
