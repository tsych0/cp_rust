// Created by Ayush Biswas at 2025/07/15 20:54
// https://cses.fi/problemset/task/1640
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashMap;

sol! {
    fn solution(
        [_n, target]: [usize; 2],
        a: [usize]
    ) -> CPResult<Words<usize>, String> {
        let mut seen = HashMap::new();
        a.into_iter().zip(1..).find_map(|(ai, i)|  {
            let bj = target.saturating_sub(ai);
            seen
                .remove(&bj)
                .map(|j| words_of(vec![j, i]))
                .or_else(|| {seen.insert(ai, i); None})
        })
        .ok_or("IMPOSSIBLE".into())
        .into()
    }
}

// @code end
