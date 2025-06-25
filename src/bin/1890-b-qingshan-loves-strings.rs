// Created by Ayush Biswas at 2025/06/06 15:03
// https://codeforces.com/problemset/problem/1890/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol_n! {
    fn solution(
        [_n, _k]: [usize; 2],
        s: [01],
        t: [01]
    ) -> bool {
        if s.iter().group_by(|&c| c).filter(|g| g.len() > 1).count() == 0 {
            true
        } else {
            if t.iter().group_by(|&c| c).filter(|g| g.len() > 1).count() > 0
                || t.first().unwrap() != t.last().unwrap() {
                false
            } else {
                let inserter = t.first().unwrap();
                s.iter()
                    .group_by(|&c| c)
                    .filter(|g| g.len() > 1)
                    .all(|g| g[0] != inserter)
            }
        }
    }
}

// @code end
