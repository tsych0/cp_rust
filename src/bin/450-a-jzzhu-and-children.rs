// Created by Ayush Biswas at 2025/07/07 16:25
// https://codeforces.com/problemset/problem/450/a
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::VecDeque;

sol! {
    fn solution(
        [n, m]: [usize; 2],
        a: [usize]
    ) -> usize {
        let mut childrens: VecDeque<_> = a.into_iter().zip(1..).collect();

        while !childrens.is_empty() {
            let (hunger, child) = childrens.pop_front().unwrap();

            if childrens.is_empty() {
                return child
            }

            if hunger > m {
                childrens.push_back((hunger - m, child));
            }
        }

        0
    }
}

// @code end
