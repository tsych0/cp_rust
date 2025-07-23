// Created by Ayush Biswas at 2025/07/17 16:24
// https://cses.fi/problemset/task/1141
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
                if let Some(dup_idx) = seen.remove(ai) {
                    for i in *start..=dup_idx {
                        seen.remove(&a[i]);
                    }
                    *start = dup_idx + 1;
                }
                seen.insert(ai, i);
                Some(seen.len())
            })
            .max()
            .unwrap()
    }
}

// @code end
