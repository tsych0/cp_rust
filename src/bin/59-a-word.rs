// Created by Ayush Biswas at 2025/05/15 12:33
// https://codeforces.com/problemset/problem/59/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn (
        s: String
    ) -> String {
        if s.chars().into_iter().filter(|c| c.is_lowercase()).count()
            < s.chars().into_iter().filter(|c| c.is_uppercase()).count() {
            s.to_uppercase()
        } else {
            s.to_lowercase()
        }
    }
}

// @code end
