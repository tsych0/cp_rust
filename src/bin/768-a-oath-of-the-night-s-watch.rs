// Created by Ayush Biswas at 2025/06/29 18:37
// https://codeforces.com/problemset/problem/768/A
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        _n: usize,
        a: [usize]
    ) -> usize {
        let f = a.into_iter().sorted().group_by(|&ai| ai).skip(1).collect::<Vec<_>>();
        let n = f.len();
        f.into_iter().take(n.saturating_sub(1)).map(|g| g.len()).sum()
    }
}

// @code end
