// Created by Ayush Biswas at 2025/07/17 16:40
// https://cses.fi/problemset/task/1073
use cp_lib::*;

// @code begin

use cpio::*;
use std::{collections::BTreeMap, ops::AddAssign};

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        let mut rounds = BTreeMap::<usize, usize>::new();
        for ai in a {
            if let Some((&key, _)) = rounds.range(ai+1..).next() {
                let v = rounds.get_mut(&key).unwrap();
                if *v == 1 {
                    rounds.remove(&key);
                } else {
                    *v -= 1;
                }
            }
            rounds.entry(ai).or_insert(0).add_assign(1);
        }
        rounds.values().sum()
    }
}

// @code end
