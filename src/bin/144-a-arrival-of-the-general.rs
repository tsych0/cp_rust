// Created by Ayush Biswas at 2025/06/03 10:27
// https://codeforces.com/problemset/problem/144/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: usize,
        a: [usize]
    ) -> usize {
        let (_, max_idx) = a
            .iter()
            .zip(0..n)
            .max_by(|(ai, idx), (aj, jdx)| ai.cmp(aj).then(jdx.cmp(idx)))
            .unwrap();

        let (_, min_idx) = a.iter().zip((0..n).rev()).min().unwrap();

        if max_idx > (n - min_idx - 1) {
            max_idx + min_idx - 1
        } else {
            max_idx + min_idx
        }
    }
}
// @code end
