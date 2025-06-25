// Created by Ayush Biswas at 2025/06/06 15:21
// https://codeforces.com/problemset/problem/1890/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn (
        n: usize,
        a: [usize]
    ) -> bool {
        let grps = a
            .into_iter()
            .sorted()
            .group_by(|&c| c)
            .map(|g| g.len())
            .collect::<Vec<_>>();
        grps.len() == 1 || (grps.len() == 2 && grps[0].abs_diff(grps[1]) < 2)
    }
}

// @code end
