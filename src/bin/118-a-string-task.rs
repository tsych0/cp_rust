// Created by Ayush Biswas at 2025/07/02 20:10
// https://codeforces.com/problemset/problem/118/A
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::iter::once;

sol! {
    fn solution(
        s: String
    ) -> String {
        let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
        once('.').chain(s.to_lowercase().chars().filter(|c| !vowels.contains(c)).intersperse('.')).collect()
    }
}

// @code end
