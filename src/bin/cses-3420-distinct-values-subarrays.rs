// Created by Ayush Biswas at 2025/07/17 22:00
// https://cses.fi/problemset/task/3420
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashMap;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        a
            .iter()
            .enumerate()
            .scan((HashMap::new(), 0), |(seen, start), (i, ai)| {
                if let Some(&dup_idx) = seen.get(ai) {
                    for i in *start..=dup_idx {
                        seen.remove(&a[i]);
                    }
                    *start = dup_idx + 1;
                }
                seen.insert(ai, i);
                Some(seen.len())
            })
            .sum()
    }
}

// @code end
