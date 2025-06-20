// Created by Ayush Biswas at 2025/06/10 15:29
// https://codeforces.com/problemset/problem/2033/B
#![allow(unused)]

use cf_rust::cpio;
use cf_rust::itertools;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> isize
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let a: Vec<Vec<isize>> = input.read_lines(n, parse_vec).unwrap();

    let mut values: HashMap<isize, isize> = HashMap::new();

    for i in 0..n {
        for j in 0..n {
            let diag = (i as isize - j as isize);
            let v = values.remove(&diag).unwrap_or(0);
            values.insert(diag, v.min(a[i][j]));
        }
    }

    -values.values().sum::<isize>()
}
// @code end
