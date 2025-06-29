// Created by Ayush Biswas at 2025/06/29 00:36
// https://codeforces.com/problemset/problem/814/A
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::cmp::Reverse;

sol! {
    fn solution(
        [_, _]: [usize; 2],
        (mut a): [usize],
        b: [usize]
    ) -> bool {
        let mut b_iter = b.into_iter().sorted_by_key(|&bi| Reverse(bi));
        for ai in a.iter_mut() {
            if ai == &0 {
                *ai = b_iter.next().unwrap();
            }
        }
        !Itertools::is_sorted(a.into_iter())
    }
}

// @code end
