// Created by Ayush Biswas at 2025/06/04 10:44
// https://codeforces.com/problemset/problem/1352/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    (
        (mut n) is usize
    ) -> Lines<Words<usize>>
    {
        let mut mask = 1;
        let mut res = vec![];
        while n > 0 {
            if n % 10 != 0 {
                res.push(n % 10 * mask);
            }
            n /= 10;
            mask *= 10;
        }
        vec![vec![res.len()].into(), ListOf(res)].into()
    }
}

// @code end
