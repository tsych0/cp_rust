// Created by Ayush Biswas at 2025/06/13 13:38
// https://codeforces.com/problemset/problem/1779/B

use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: isize
    ) -> Lines<String> {
        if n % 2 == 0 {
            vec![
                "YES".into(),
                ListOf::<' ', _>(vec![1, -1].into_iter().cycle().take(n as usize).collect())
                    .to_string(),
            ]
            .into()
        } else if n >= 5 {
            vec![
                "YES".into(),
                ListOf::<' ', _>(
                    vec![(n - 2) / 2, -((n - 2) / 2 + 1)]
                        .into_iter()
                        .cycle()
                        .take(n as usize)
                        .collect(),
                )
                .to_string(),
            ]
            .into()
        } else {
            ListOf(vec!["NO".into()])
        }
    }
}

// @code end
