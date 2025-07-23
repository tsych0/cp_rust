// Created by Ayush Biswas at 2025/07/10 20:24
// https://cses.fi/problemset/task/1622
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::iter::once;

sol! {
    fn solution(
        (mut s): [char]
    ) -> Lines<String> {
        let res = permutations(&mut s, 0).into_iter().unique().sorted().collect_vec();
        once(res.len().to_string()).chain(res.into_iter()).collect()
    }
}

fn permutations(s: &mut Vec<char>, i: usize) -> Vec<String> {
    if i == s.len() {
        return vec![s.iter().collect()];
    }

    (i..s.len())
        .map(|j| {
            if s[i] == s[j] {
                permutations(s, i + 1)
            } else {
                s.swap(i, j);
                let res = permutations(s, i + 1);
                s.swap(i, j);
                res
            }
        })
        .collect::<Vec<_>>()
        .concat()
}

// @code end
