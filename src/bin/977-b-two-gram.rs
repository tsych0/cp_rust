// Created by Ayush Biswas at 2025/06/28 19:47
// https://codeforces.com/problemset/problem/977/B
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::collections::HashMap;

sol! {
    fn solution(
        n: usize,
        s: [char]
    ) -> String {
        let mut two_grams = HashMap::new();
        for i in 1..n {
            let two: String = s.get(i-1..=i).unwrap().into_iter().collect();
            if two_grams.contains_key(&two) {
                let curr_val = two_grams.remove(&two).unwrap();
                two_grams.insert(two, curr_val + 1);
            } else {
                two_grams.insert(two, 1);
            }
        }

        two_grams.into_iter().sorted_by_key(|(_, v)| v.clone()).last().unwrap().0
    }
}

// @code end
