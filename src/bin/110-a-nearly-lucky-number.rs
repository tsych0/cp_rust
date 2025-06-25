// Created by Ayush Biswas at 2025/05/16 12:05
// https://codeforces.com/problemset/problem/110/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        n: String
    ) -> bool {
        n.chars()
            .filter(|&c| c == '4' || c == '7')
            .count()
            .to_string()
            .chars()
            .all(|c| c == '4' || c == '7')
    }
}
// @code end
