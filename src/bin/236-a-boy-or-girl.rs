// Created by Ayush Biswas at 2025/05/14 10:43
// https://codeforces.com/problemset/problem/236/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashSet;

sol! {
    fn solution(
        s: String
    ) -> String {
        if s.chars().collect::<HashSet<_>>().len() % 2 == 0 {
            "CHAT WITH HER!".into()
        } else {
            "IGNORE HIM!".into()
        }
    }
}

// @code end
