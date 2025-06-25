// Created by Ayush Biswas at 2025/06/14 18:07
// https://codeforces.com/problemset/problem/1699/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    fn (
        [n, m]: [usize; 2]
    ) -> Lines<Words<usize>> {
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
}

// @code end
