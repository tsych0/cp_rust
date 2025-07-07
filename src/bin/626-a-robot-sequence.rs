// Created by Ayush Biswas at 2025/07/06 12:26
// https://codeforces.com/problemset/problem/626/A
use cp_lib::*;

// @code begin
use cpio::*;
use std::{collections::HashMap, ops::AddAssign};

sol! {
    fn solution(_n: usize, direction: [char]) -> usize {
        let dir = |c: char| match c {
            'U' => (0, 1),
            'D' => (0, -1),
            'R' => (1, 0),
            'L' => (-1, 0),
            _ => (0, 0),
        };

        direction
            .into_iter()
            .fold(
                ((0, 0), HashMap::from([((0, 0), 1)])),
                |((x, y), mut count), d| {
                    let (dx, dy) = dir(d);
                    let (x, y) = (x + dx, y + dy);
                    count.entry((x, y)).or_insert(0).add_assign(1);
                    ((x, y), count)
                },
            )
            .1
            .values()
            .into_iter()
            .map(|i| (i * (i - 1)) / 2)
            .sum()
    }
}

// @code end
