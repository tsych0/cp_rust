// Created by Ayush Biswas at 2025/06/20 15:09
// https://codeforces.com/problemset/problem/1823/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::collections::HashSet;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> isize
where
    R: Read,
{
    let [n, k]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let p: Vec<usize> = input.read_line(parse_vec).unwrap();
    let x = p
        .into_iter()
        .enumerate()
        .sorted_by_key(|(i, j)| i % k)
        .group_by(|(i, j)| i % k)
        .map(|g| g.into_iter().map(|(i, j)| j).collect::<HashSet<_>>());
    let y = (1..=n)
        .enumerate()
        .sorted_by_key(|(i, j)| i % k)
        .group_by(|(i, j)| i % k)
        .map(|g| g.into_iter().map(|(i, j)| j).collect::<HashSet<_>>());

    let faults = x.zip(y).map(|(xi, yi)| xi.difference(&yi).count()).sum();

    match faults {
        0 => 0,
        2 => 1,
        _ => -1,
    }
}
// @code end
