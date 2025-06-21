// Created by Ayush Biswas at 2025/06/14 11:57
// https://codeforces.com/problemset/problem/1845/B
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
    let [xa, ya]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let [xb, yb]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let [xc, yc]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();

    let mut res = 0;
    if xa > xb && xa > xc || xa < xb && xa < xc {
        res += xa.abs_diff(xb).min(xa.abs_diff(xc));
    }

    if ya > yb && ya > yc || ya < yb && ya < yc {
        res += ya.abs_diff(yb).min(ya.abs_diff(yc));
    }

    res + 1
}
// @code end
