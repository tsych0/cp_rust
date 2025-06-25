// Created by Ayush Biswas at 2025/06/13 10:39
// https://codeforces.com/problemset/problem/1860/A

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        s: String
    ) -> Lines<String> {
        let n = s.len();
        let r = vec!["()".repeat(n), "(".repeat(n) + &")".repeat(n)];
        match r.into_iter().find(|ri| !ri.contains(&s)) {
            Some(r) => ListOf(vec!["YES".into(), r]),
            None => ListOf(vec!["NO".into()]),
        }
    }
}

// @code end
