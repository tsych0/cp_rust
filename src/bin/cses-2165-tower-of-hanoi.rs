// Created by Ayush Biswas at 2025/07/10 20:11
// https://cses.fi/problemset/task/2165
use cp_lib::*;

// @code begin
use cpio::*;
use std::iter::once;

sol! {
    fn solution(
        n: usize,
    ) -> Lines<Words<usize>> {
        let res = tower(n, 1, 3);
        once(words_of(vec![res.len()]))
            .chain(res.into_iter().map(|v| words_of(v.into())))
            .collect()
    }
}

fn tower(size: usize, source: usize, destination: usize) -> Vec<[usize; 2]> {
    if size == 0 {
        return vec![];
    }

    let auxillary = 6 - source - destination;
    let move_top = tower(size - 1, source, auxillary);
    let this_move = [source, destination];
    let move_rest = tower(size - 1, auxillary, destination);

    move_top
        .into_iter()
        .chain(once(this_move))
        .chain(move_rest.into_iter())
        .collect()
}

// @code end
