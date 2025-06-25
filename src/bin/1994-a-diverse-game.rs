// Created by Ayush Biswas at 2025/05/19 16:36
// https://codeforces.com/problemset/problem/1994/A
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        [n, m]: [usize; 2],
        matrix: [[isize]]; n
    ) -> Lines<Words<isize>> {
        let mut matrix = matrix;
        if n != 1 {
             let x = matrix.pop().unwrap();
             matrix.insert(0, x);
             matrix.into_iter().map(|v| ListOf(v)).collect()
         } else if m != 1 {
             for i in 0..n {
                 let y = matrix[i].pop().unwrap();
                 matrix[i].insert(0, y);
             }
             matrix.into_iter().map(|v| ListOf(v)).collect()
         } else {
             ListOf(vec![ListOf(vec![-1])])
         }
    }
}

// @code end
