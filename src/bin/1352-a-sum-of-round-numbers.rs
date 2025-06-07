// Created by Ayush Biswas at 2025/06/04 10:44
// https://codeforces.com/problemset/problem/1352/A
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

fn solution<R>(input: &mut CPInput<R>) -> ListOf<ListOf<usize>>
where
    R: Read,
{
    let mut n: usize = input.read_line(parse).unwrap();
    let mut mask = 1;
    let mut res = vec![];
    while n > 0 {
        if n % 10 != 0 {
            res.push(n % 10 * mask);
        }
        n /= 10;
        mask *= 10;
    }
    ListOf::LinesOf(vec![ListOf::WordsOf(vec![res.len()]), ListOf::WordsOf(res)])
}
// @code end
