// Created by Ayush Biswas at 2025/07/13 12:22
// https://cses.fi/problemset/task/1743
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::{
    collections::HashMap,
    ops::{AddAssign, SubAssign},
};

sol! {
    fn solution(s: [char]) -> CPResult<String, i8> {
        let n = s.len();
        let mut count: HashMap<char, usize> = s
            .into_iter()
            .sorted()
            .group_by(|&si| si)
            .map(|g| (g[0], g.len()))
            .collect();

        let m = mode(&count);
        if m > n.div_ceil(2) {
            return Err(-1).into()
        }

        let mut res = vec!['.'; n];
        let mut prev = '.';

       'l:
        for i in 0..n {
            for c in ('A'..='Z').filter(|&d| prev != d)  {
                if *count.get(&c).unwrap_or(&0) > 0 {
                    count.entry(c).or_insert(0).sub_assign(1);
                    let m = mode(&count);
                    if m > (n-i-1).div_ceil(2) {
                        count.entry(c).or_insert(0).add_assign(1);
                    } else {
                        prev = c;
                        res[i] = c;
                        continue 'l;
                    }
                }
            }
            return Err(-1).into()
        }

        Ok(res.into_iter().collect()).into()
    }
}

#[inline(always)]
fn mode(count: &HashMap<char, usize>) -> usize {
    count.into_iter().max_by_key(|&(_, v)| v).unwrap().1.clone()
}

// @code end
