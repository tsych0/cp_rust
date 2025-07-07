// Created by Ayush Biswas at 2025/07/01 11:39
// https://codeforces.com/problemset/problem/2025/C
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::{collections::HashMap, ops::AddAssign};

sol_n! {
    fn solution(
        [n, k]: [usize; 2],
        a: [usize]
    ) -> usize {
        let mut count = HashMap::new();
        for ai in a {
            count.entry(ai).or_insert(0).add_assign(1);
        }
        let a = count.keys().cloned().sorted().collect_vec();
        let array = a.iter().map(|k| count.get(k).unwrap().clone()).into_iter()
            .fold((vec![0], 0), |(mut acc, sum), i| {
                let sum_new = sum + i;
                acc.push(sum_new);
                (acc, sum_new)
            }).0;

        let n = a.len();
        let mut res = 0;
        let (mut i, mut j) = (0, 0);
        while j < n {
            while j - i + 1 > k {
                i += 1;
            }
            if a[j] - a[i] == j - i {
                res = res.max(array[j+1] - array[i]);
                j += 1;
            } else {
                i += 1;
            }
        }
        res
    }
}

// @code end
