// Created by Ayush Biswas at 2025/07/21 15:09
// https://cses.fi/problemset/task/3359
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        grid: [[char]; n]
    ) -> String {
        let mut prev = vec![];
        let mut curr = vec![];

        let mut res = vec![' '; 2 * n - 1];

        curr.push((grid[0][0], (0, 0)));

        let mut visited = vec![vec![false; n + 1]; n + 1];
        visited[0][0] = true;

        for level in 1..2*n {
            let temp = prev;
            prev = curr;
            curr = temp;

            curr.clear();

            let min = prev.iter().min_by_key(|(k, _)| k).unwrap().0;
            res[level - 1] = min;

            for &(_, (i, j)) in prev.iter().filter(|(k, _)| *k == min) {
                if i < n - 1 && !visited[i + 1][j] {
                    visited[i + 1][j] = true;
                    curr.push((grid[i + 1][j], (i + 1, j)));
                }
                if j < n - 1 && !visited[i][j + 1] {
                    visited[i][j + 1] = true;
                    curr.push((grid[i][j + 1], (i, j + 1)));
                }
            }
        }

        // println!("{:?}", visited);

        res.into_iter().collect()
        // "".into()
    }
}

// @code end
