// Created by Ayush Biswas at 2025/07/14 15:16
// https://cses.fi/problemset/task/1084
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, m, k]: [usize; 3],
        (mut a): [usize],
        (mut b): [usize]
    ) -> usize {
        a.sort();
        b.sort();
        let (mut i, mut j) = (0, 0);

        let mut res = 0;
        while i < n && j < m {
            if a[i] < b[j].saturating_sub(k) {
                i += 1;
            } else if a[i] > b[j] + k {
                j += 1;
            } else {
                res += 1;
                i += 1;
                j += 1;
            }
        }

        res
    }
}

// @code end
