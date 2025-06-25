// Created by Ayush Biswas at 2025/06/10 15:29
// https://codeforces.com/problemset/problem/2033/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashMap;

sol! {
    fn solution(
        n: usize,
        a: [[isize]]; n
    ) -> isize {
        let mut values: HashMap<isize, isize> = HashMap::new();

        for i in 0..n {
            for j in 0..n {
                let diag = i as isize - j as isize;
                let v = values.remove(&diag).unwrap_or(0);
                values.insert(diag, v.min(a[i][j]));
            }
        }

        -values.values().sum::<isize>()
    }
}

// @code end
