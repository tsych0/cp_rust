// Created by Ayush Biswas at 2025/06/13 11:59
// https://codeforces.com/problemset/problem/1856/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    (
        n is usize,
        a is [usize]
    ) -> bool
    {
        let one_count = a.iter().filter(|&&i| i == 1).count();
        let other_sum = a.iter().filter(|&&i| i != 1).sum::<usize>();
        let remaining_idx = n - one_count;

        other_sum - remaining_idx + one_count >= 2 * one_count && n > 1
    }
}

// @code end
