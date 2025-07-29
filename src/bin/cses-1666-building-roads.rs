// Created by Ayush Biswas at 2025/07/25 19:53
// https://cses.fi/problemset/task/1666
use cp_lib::*;
// @code begin
use cpio::*;
use dsu::Dsu;
use std::iter::once;

sol! {
    fn solution(
        [n, m]: [usize; 2],
        roads: [[usize; 2]; m]
    ) -> Lines<Words<usize>> {
        let mut dsu = Dsu::new(n);
        for [city1, city2] in roads {
            dsu.merge(city1 - 1, city2 - 1);
        }

        let grps = dsu.groups();

        once(vec![grps.len() - 1])
        .chain(
                grps.windows(2).map(|w| vec![w[0][0] + 1, w[1][0] + 1].into())
            ).map(words_of).collect()
    }
}

// @code end
