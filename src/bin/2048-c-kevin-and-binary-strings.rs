// Created by Ayush Biswas at 2025/06/30 22:14
// https://codeforces.com/problemset/problem/2048/C
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol_n! {
    fn solution(
        s: [01]
    ) -> Words<usize> {
        let n = s.len();
        let first_two_grps = s.into_iter().group_by(|&si| si).take(2).collect_vec();
        let one_count = first_two_grps[0].len();
        if one_count == n {
            return vec![1, n, 1, 1].into()
        }
        let zero_count = first_two_grps[1].len();
        let shift = std::cmp::min(one_count, zero_count);
        vec![1, n, one_count - shift + 1, n - shift].into()
    }
}

// @code end
