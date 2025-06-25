// Created by Ayush Biswas at 2025/06/09 00:00
// https://codeforces.com/problemset/problem/208/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        s: String
    ) -> String {
        s.split("WUB")
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

// @code end
