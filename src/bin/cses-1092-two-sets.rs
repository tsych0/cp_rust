// Created by Ayush Biswas at 2025/07/10 11:39
// https://cses.fi/problemset/task/1092
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::ops::AddAssign;

sol! {
    fn solution(n: usize) -> Lines<String> {
        let sum = if n % 2 == 0 {
            (n / 2) * (n + 1)
        } else {
            n * ((n + 1) / 2)
        };

        if sum % 2 != 0 {
            return vec!["NO".into()].into();
        }

        let s = sum / 2;
        let j = s / n;
        let last_element = n - j + 1;
        let first_set = (last_element..=n)
            .chain(1..)
            .scan(0, |sum, ai| {
                sum.add_assign(ai);
                if *sum <= s {
                    Some(ai)
                } else {
                    None
                }
            })
            .collect_vec();

        let first_element = if first_set.last().unwrap() < &last_element {
            first_set.last().unwrap() + 1
        } else {
            1
        };

        let second_set = (first_element..last_element).collect_vec();

        vec![
            "YES".into(),
            first_set.len().to_string(),
            words_of(first_set).cp_fmt(),
            second_set.len().to_string(),
            words_of(second_set).cp_fmt(),
        ]
        .into()
    }
}

// @code end
