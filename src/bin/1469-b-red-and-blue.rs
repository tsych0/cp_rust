// Created by Ayush Biswas at 2025/07/05 12:04
// https://codeforces.com/problemset/problem/1469/B
use cp_lib::*;

// @code begin
use cpio::*;
use std::ops::AddAssign;

sol_n! {
    fn solution(
        n: usize,
        r: [isize],
        m: usize,
        b: [isize]
    ) -> isize {
        let r_tak = r.into_iter().scan(0, |acc, ri| {
            acc.add_assign(ri);
            Some(*acc)
        }).max().unwrap().max(0);

        let l_tak = b.into_iter().scan(0, |acc, bi| {
            acc.add_assign(bi);
            Some(*acc)
        }).max().unwrap().max(0);

        r_tak + l_tak
    }
}

// @code end
