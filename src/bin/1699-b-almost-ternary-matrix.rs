// Created by Ayush Biswas at 2025/06/14 18:07
// https://codeforces.com/problemset/problem/1699/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Lines<Words<usize>>
where
    R: Read,
{
    println!("{}", '\n');
    let [n, m]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let f: Vec<usize> = [0, 1, 1, 0]
        .into_iter()
        .cycle()
        .take(m)
        .map(|v| v.clone())
        .collect::<Vec<_>>();
    let s: Vec<usize> = [1, 0, 0, 1]
        .into_iter()
        .cycle()
        .take(m)
        .map(|v| v.clone())
        .collect::<Vec<_>>();

    vec![f.clone(), s.clone(), s, f]
        .into_iter()
        .cycle()
        .take(n)
        .map(ListOf)
        .collect()
}
// @code end
