// Created by Ayush Biswas at 2025/07/06 14:23
// https://codeforces.com/problemset/problem/1520/c
use cp_lib::*;

// @code begin
use cpio::*;

sol_n! {
    fn solution(
        n: usize,
    ) -> Result<Lines<Words<usize>>> {
        if n == 2 {
            return Err("-1".into());
        }

        let mut matrix = vec![vec![0; n]; n];
        let mut start = (0, 0);
        let mut dir = [(0, 1), (1, 0)].into_iter().cycle();
        let mut scaler = 1;
        let mut curr = 1;
        let m = n*n;
        while curr <= m {
            let (x, y) = start;
            for i in 0.. {
                if x+i < n && y+i < n {
                    matrix[x+i][y+i] = curr;
                    curr += 1;
                } else {
                    break;
                }
            }
            scaler += 1;
            let (dx, dy) = dir.next().unwrap();
            start = (dx * (scaler / 2), dy * (scaler / 2));
        }

        Ok(matrix.into_iter().map(|l| l.into_iter().collect()).collect())
    }
}

// @code end
