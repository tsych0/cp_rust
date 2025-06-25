// Created by Ayush Biswas at 2025/05/14 18:01
// https://codeforces.com/problemset/problem/281/A
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        s: String
    ) -> String {
        let mut c = s.chars();
        format!(
            "{}{}",
            c.next().unwrap().to_uppercase(),
            c.collect::<String>()
        )
    }
}

// @code end
