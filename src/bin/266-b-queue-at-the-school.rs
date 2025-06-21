// Created by Ayush Biswas at 2025/05/19 09:32
// https://codeforces.com/problemset/problem/266/B
use cp_rust::*;

// @code begin
use cpio::*;
#[allow(unused)]
use std::convert::TryInto;
use std::io::Read;

fn main() {
    solve(solution)
}

fn transform(queue: &[char]) -> Vec<char> {
    match queue {
        ['B', 'G', rest @ ..] => {
            let mut result = vec!['G', 'B'];
            result.extend(transform(rest));
            result
        }
        [c, rest @ ..] => {
            let mut result = vec![*c];
            result.extend(transform(rest));
            result
        }
        [] => Vec::new(), // Handle empty slice case
    }
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let [_, t]: [usize; 2] = input.read_line(parse_vec).unwrap().try_into().unwrap();
    let s: String = input.read_line(parse).unwrap();
    let mut s = s.chars().collect::<Vec<_>>();
    for _ in 0..t {
        s = transform(&s)
    }

    s.into_iter().collect()
}
// @code end
