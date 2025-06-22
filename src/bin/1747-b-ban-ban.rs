// Created by Ayush Biswas at 2025/06/14 14:12
// https://codeforces.com/problemset/problem/1747/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Lines<Words<usize>>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let count = (n + 1) / 2;
    let s = "BAN".repeat(n);
    let b = s
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == 'B')
        .map(|(i, _)| i)
        .take(count);
    let a = s
        .chars()
        .enumerate()
        .filter(|&(_, c)| c == 'N')
        .map(|(i, _)| i)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .take(count);
    let mut res: Vec<_> = b.zip(a).map(|(b, a)| ListOf(vec![b + 1, a + 1])).collect();
    res.insert(0, ListOf(vec![res.len()]));
    ListOf(res)
}
// @code end
