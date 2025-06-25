// Created by Ayush Biswas at 2025/06/09 12:53
// https://codeforces.com/problemset/problem/1826/D

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        b: [isize]
    ) -> isize {
        let mut max_result = 0;
        let left = b
            .iter()
            .enumerate()
            .skip(1)
            .fold(vec![b[0]], |mut acc: Vec<isize>, (i, bi)| {
                acc.push(acc[i - 1].max(i as isize + bi));
                acc
            });
        let mut right = b.iter().enumerate().take(n - 1).rfold(
            vec![b[n - 1] - (n - 1) as isize],
            |mut acc: Vec<isize>, (i, bi)| {
                acc.push(acc[n - i - 2].max(bi - i as isize));
                acc
            },
        );
        right.reverse();

        for i in 1..n - 1 {
            max_result = max_result.max(left[i - 1] + right[i + 1] + b[i]);
        }

        max_result
    }
}

// @code end
