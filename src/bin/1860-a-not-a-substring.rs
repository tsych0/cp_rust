// Created by Ayush Biswas at 2025/06/13 10:39
// https://codeforces.com/problemset/problem/1860/A

use cp_lib::*;

// @code begin
use cpio::*;
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
    let r = vec!["()".repeat(n), "(".repeat(n) + &")".repeat(n)];
    match r.into_iter().find(|ri| !ri.contains(&s)) {
        Some(r) => ListOf(vec!["YES".into(), r]),
        None => ListOf(vec!["NO".into()]),
    }
}
// @code end
