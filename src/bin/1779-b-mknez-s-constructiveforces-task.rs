// Created by Ayush Biswas at 2025/06/13 13:38
// https://codeforces.com/problemset/problem/1779/B

use cp_lib::*;

// @code begin
use crate::itertools::Itertools;
use cpio::*;
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve_n(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> Lines<String>
where
    R: Read,
{
    let n: isize = input.read_line(parse).unwrap();
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
// @code end
