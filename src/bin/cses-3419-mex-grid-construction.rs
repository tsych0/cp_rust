// Created by Ayush Biswas at 2025/07/12 12:50
// https://cses.fi/problemset/task/3419
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashSet;

sol! {
    fn solution(
        n: usize,
    ) -> Lines<Words<usize>> {
        let mut matrix = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n {
                let mut set = HashSet::new();
                for k in 0..i {
                    set.insert(matrix[k][j]);
                }
                for l in 0..j {
                    set.insert(matrix[i][l]);
                }
                matrix[i][j] = (0..).find(|i| !set.contains(i)).unwrap();
            }
        }
        matrix.into_iter().map(|v| words_of(v)).collect()
    }
}

// @code end
