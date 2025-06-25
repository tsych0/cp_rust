// Created by Ayush Biswas at 2025/06/06 17:10
// https://codeforces.com/problemset/problem/467/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        pq: [[usize]]; n
    ) -> usize {
        pq.into_iter().filter(|pqi| pqi[1] - pqi[0] >= 2).count()
    }
}

// @code end
