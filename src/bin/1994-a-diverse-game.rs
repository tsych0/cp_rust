// Created by Ayush Biswas at 2025/05/19 16:36
// https://codeforces.com/problemset/problem/1994/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, m]: [usize; 2],
        matrix: [[usize]; n]
    ) -> CPResult<Lines<Words<usize>>, i8> {
        let mut matrix = matrix;
        if n != 1 {
            let x = matrix.pop().unwrap();
            matrix.insert(0, x);
            Ok(matrix.into_iter().map(|v| ListOf(v)).collect())
        } else if m != 1 {
            for i in 0..n {
                let y = matrix[i].pop().unwrap();
                matrix[i].insert(0, y);
            }
            Ok(matrix.into_iter().map(|v| ListOf(v)).collect())
        } else {
            Err(-1)
        }.into()
    }
}

// @code end
