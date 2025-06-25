// Created by Ayush Biswas at 2025/06/12 15:49
// https://codeforces.com/problemset/problem/1951/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn (
        n: usize,
        s: [01]
    ) -> bool {
        let one_count = s.iter().filter(|&&i| i == 1).count();
        if one_count % 2 == 0 && one_count != 2 {
            true
        } else if one_count == 2 {
            s.into_iter().group_by(|&c| c).filter(|g| g[0] == 1).count() == 2
        } else {
            false
        }
    }
}

// @code end
