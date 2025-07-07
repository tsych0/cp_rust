// Created by Ayush Biswas at 2025/07/05 11:23
// https://codeforces.com/problemset/problem/1051/B
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;

sol! {
    fn solution(
        [l, r]: [usize; 2]
    ) -> Lines<Words<usize>> {
        println!("YES");
        (l..=r)
            .collect_vec()
            .windows(2)
            .step_by(2)
            .map(|w| words_of(w.to_vec()))
            .collect()
    }
}

// @code end
