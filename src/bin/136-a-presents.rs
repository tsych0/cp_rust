// Created by Ayush Biswas at 2025/05/20 11:24
// https://codeforces.com/problemset/problem/136/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: usize,
        p: [usize]
    ) -> Words<usize> {
        let mut res = vec![0; n];
        for i in 0..n {
            res[p[i] - 1] = i + 1;
        }
        res.into()
    }
}

// @code end
