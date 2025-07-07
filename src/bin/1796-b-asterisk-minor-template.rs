// Created by Ayush Biswas at 2025/07/04 13:10
// https://codeforces.com/problemset/problem/1796/B
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::collections::HashSet;

sol_n! {
    fn solution(
        a: [char],
        b: [char]
    ) -> Lines<String> {
        if a[0] == b[0]  {
            vec!["YES".into(), format!("{}*", a[0])]
        } else if a.last().unwrap() == b.last().unwrap() {
            vec!["YES".into(), format!("*{}", a.last().unwrap())]
        } else {
            let a = a.windows(2).collect::<HashSet<_>>();
            let b = b.windows(2).collect();
            let section = a.intersection(&b).collect_vec();
            if section.len() != 0 {
                vec!["YES".into(), format!("*{}*", section[0].into_iter().collect::<String>())]
            } else {
                vec!["NO".into()]
            }
        }.into()
    }
}

// @code end
