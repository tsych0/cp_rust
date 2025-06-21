// Created by Ayush Biswas at 2025/06/19 10:36
// https://codeforces.com/problemset/problem/474/A
#![allow(unused)]

use cp_rust::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::Read;
use std::ops::Add;
use std::ops::Sub;

fn main() {
    solve(solution)
}

fn solution<R>(input: &mut CPInput<R>) -> String
where
    R: Read,
{
    let dir: char = input.read_line(parse).unwrap();
    let s: String = input.read_line(parse).unwrap();
    let keyboard = vec!["qwertyuiop", "asdfghjkl;", "zxcvbnm,./"];
    let char_map: HashMap<char, (usize, usize)> = keyboard
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| (c, (i, j)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .concat()
        .into_iter()
        .collect();
    let key_map: HashMap<(usize, usize), char> = keyboard
        .into_iter()
        .enumerate()
        .map(|(i, row)| {
            row.chars()
                .enumerate()
                .map(|(j, c)| ((i, j), c))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
        .concat()
        .into_iter()
        .collect();

    let mv = if dir == 'L' {
        |x: usize| x.add(1)
    } else {
        |x: usize| x.sub(1)
    };

    s.chars()
        .map(|c| {
            let &(r, c) = char_map.get(&c).unwrap();
            key_map.get(&(r, mv(c))).unwrap()
        })
        .collect()
}
// @code end
