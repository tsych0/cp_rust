// Created by Ayush Biswas at 2025/06/08 12:42
// https://codeforces.com/problemset/problem/2072/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        _n: usize,
        s: String
    ) -> usize {
        let a = s.chars().filter(|&c| c == '_').count();
        let b = s.chars().filter(|&c| c == '-').count();
        ((b + 1) / 2) * (b / 2) * a
    }
}
// @code end
