// Created by Ayush Biswas at 2025/06/13 10:39
// https://codeforces.com/problemset/problem/1860/A
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

fn solution<R>(input: &mut CPInput<R>) -> Lines<String>
where
    R: Read,
{
    let s: String = input.read_line(parse).unwrap();
    let n = s.len();
    let mut r = vec!["()".repeat(n), "(".repeat(n) + &")".repeat(n)];
    match r.into_iter().find(|ri| !ri.contains(&s)) {
        Some(r) => ListOf(vec!["YES".into(), r]),
        None => ListOf(vec!["NO".into()]),
    }
}
// @code end
