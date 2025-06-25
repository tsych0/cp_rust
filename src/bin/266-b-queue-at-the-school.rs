// Created by Ayush Biswas at 2025/05/19 09:32
// https://codeforces.com/problemset/problem/266/B
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        [_, t]: [usize; 2],
        s: String
    ) -> String {
        let mut s = s.chars().collect::<Vec<_>>();
        for _ in 0..t {
            s = transform(&s)
        }

        s.into_iter().collect()
    }
}

fn transform(queue: &[char]) -> Vec<char> {
    match queue {
        ['B', 'G', rest @ ..] => {
            let mut result = vec!['G', 'B'];
            result.extend(transform(rest));
            result
        }
        [c, rest @ ..] => {
            let mut result = vec![*c];
            result.extend(transform(rest));
            result
        }
        [] => Vec::new(), // Handle empty slice case
    }
}
// @code end
