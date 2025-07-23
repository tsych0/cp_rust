// Created by Ayush Biswas at 2025/07/10 12:18
// https://cses.fi/problemset/task/1755
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::{collections::HashMap, iter::repeat, ops::AddAssign};

sol! {
    fn solution(s: [char]) -> Result<String> {
        let count = s.into_iter().sorted().fold(HashMap::new(), |mut count, c| {
            count.entry(c).or_insert(0).add_assign(1);
            count
        });
        if count.values().filter(|&v| v % 2 == 1).count() > 1 {
            return Err("NO SOLUTION".into());
        }

        let side: String = count
            .iter()
            .filter_map(|(&k, v)| {
                if v % 2 == 0 {
                    Some(repeat(k).take(v / 2).collect::<String>())
                } else {
                    None
                }
            })
            .collect();
        let mid: String = count
            .into_iter()
            .filter_map(|(k, v)| {
                if v % 2 == 1 {
                    Some(repeat(k).take(v).collect::<String>())
                } else {
                    None
                }
            })
            .collect();

        Ok(format!("{}{}{}", side, mid, side.chars().rev().collect::<String>()))
    }
}

// @code end
