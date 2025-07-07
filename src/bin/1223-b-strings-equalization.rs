// Created by Ayush Biswas at 2025/07/04 12:27
// https://codeforces.com/problemset/problem/1223/B
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashSet;

sol_n! {
    fn solution(
        s: [char],
        t: [char]
    ) -> bool {
        let s = s.into_iter().collect::<HashSet<_>>();
        let t = t.into_iter().collect::<HashSet<_>>();
        s.intersection(&t).count() != 0
    }
}

// @code end
