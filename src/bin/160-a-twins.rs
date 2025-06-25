// Created by Ayush Biswas at 2025/06/08 12:13
// https://codeforces.com/problemset/problem/160/A

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        let total: usize = a.iter().sum();
        let mut sum = 0;
        for (i, ai) in a.into_iter().sorted_by(|a, &b| b.cmp(a)).enumerate() {
            if 2 * sum > total {
                return i;
            }
            sum += ai;
        }
        n
    }
}
// @code end
