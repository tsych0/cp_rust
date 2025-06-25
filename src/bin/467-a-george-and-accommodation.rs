// Created by Ayush Biswas at 2025/06/06 17:10
// https://codeforces.com/problemset/problem/467/A

use cp_lib::*;

// @code begin
use cpio::*;
use std::io::Read;

sol! {
    (
        n is usize,
        pq is [[usize]]; n
    ) -> usize
    {
        pq.into_iter().filter(|pqi| pqi[1] - pqi[0] >= 2).count()
    }
}

// @code end
