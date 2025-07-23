// Created by Ayush Biswas at 2025/07/15 20:01
// https://cses.fi/problemset/task/1629
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        (mut movies): [[usize; 2]; n]
    ) -> usize {
        movies.sort_by_key(|[_, end]| *end);
        let mut res = 0;
        let mut last = 0;
        for [s, e] in movies {
            if last == 0 || s >= last {
                res += 1;
                last = e;
            }
        }
        res
    }
}

// @code end
