// Created by Ayush Biswas at 2025/06/21 11:12
// https://codeforces.com/problemset/problem/1436/B

use cp_lib::*;

// @code begin
use cpio::*;
use sieve::Sieve;

sol! {
    fn solution(
        n: usize
    ) -> Lines<Words<usize>> {
        let sive = Sieve::new(400);
        let next_prime = (n..)
            .find(|&i| sive.is_prime(i) && !sive.is_prime(i - n + 1))
            .unwrap();
        let diag = next_prime - n + 1;
        (0..n)
            .map(|i| (0..n).map(|j| if i == j { diag } else { 1 }).collect())
            .collect()
    }
}

// @code end
