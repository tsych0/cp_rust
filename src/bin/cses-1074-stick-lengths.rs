// Created by Ayush Biswas at 2025/07/15 21:35
// https://cses.fi/problemset/task/1074
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        (mut a): [usize]
    ) -> usize {
        a.sort();
        if n % 2 != 0 {
            let mid = a[n/2];
            a.into_iter().map(|ai| ai.abs_diff(mid)).sum()
        } else {
            let mid = (a[n/2]+a[n.div_ceil(2)])/2;
            std::cmp::min(
                a.iter().map(|ai| ai.abs_diff(mid)).sum(),
                a.iter().map(|ai| ai.abs_diff(mid + 1)).sum(),
            )
        }
    }
}

// @code end
