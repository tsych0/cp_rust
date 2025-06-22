// Created by Ayush Biswas at 2025/06/21 11:12
// https://codeforces.com/problemset/problem/1436/B

use cp_lib::*;

// @code begin
use cpio::*;
use sieve::Sieve;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Lines<Words<usize>>
where
    R: Read,
{
    let n: usize = input.read_line(parse).unwrap();
    let sive = Sieve::new(400);
    let next_prime = (n..)
        .find(|&i| sive.is_prime(i) && !sive.is_prime(i - n + 1))
        .unwrap();
    let diag = next_prime - n + 1;
    (0..n)
        .map(|i| (0..n).map(|j| if i == j { diag } else { 1 }).collect())
        .collect()
}
// @code end
