// Created by Ayush Biswas at 2025/07/04 13:22
// https://codeforces.com/problemset/problem/644/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        [n, a, b]: [isize; 3]
    ) -> Lines<Words<isize>> {
        let matrix = (0..a).scan(vec![1, 2], |state, i| {
            (0..b).map(|j| {
                let c = (i + j) % 2;
                if state[c as usize] <= n {
                    let res = state[c as usize];
                    state[c as usize] += 2;
                    Some(res)
                } else {
                    Some(0)
                }
            }).collect()
        }).collect::<Lines<Words<isize>>>();

        if *matrix.0.iter().map(|m| m.0.iter().max().unwrap()).max().unwrap() != n {
            vec![vec![-1].into()].into()
        } else {
            matrix
        }
    }
}

// @code end
