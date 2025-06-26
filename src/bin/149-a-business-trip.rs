// Created by Ayush Biswas at 2025/06/26 16:45
// https://codeforces.com/problemset/problem/149/A
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        k: usize,
        a: [usize]
    ) -> isize {
        let a: Vec<_> = a.into_iter().sorted().collect();
        let r = a.into_iter().rev().try_fold((0, 0), |(count, sum), ai| {
            if sum >= k {
                Err(count as isize)
            } else {
                Ok((count+1, sum+ai))
            }
        });
        match r {
            Ok((c, s)) => {if s >= k {c} else {-1}},
            Err(c) => c
        }
    }
}

// @code end
