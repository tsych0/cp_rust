// Created by Ayush Biswas at 2025/06/14 16:43
// https://codeforces.com/problemset/problem/1705/B

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    fn (
        n: usize,
        a: [usize]
    ) -> usize {
        let zero_count = a
            .iter()
            .take(n - 1)
            .skip_while(|&&i| i == 0)
            .filter(|&&i| i == 0)
            .count();
        a.iter().sum::<usize>() - a[n - 1] + zero_count
    }
}

// @code end
