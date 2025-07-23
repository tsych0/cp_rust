// Created by Ayush Biswas at 2025/07/22 11:44
// https://cses.fi/problemset/task/1097
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        a: [isize]
    ) -> isize {
        let mut res = vec![vec![(0, 0); n + 2]; n + 2];

        let player1 = n % 2;

        for i in (1..=n).rev() {
            for j in 1..=n {
                if i > j {
                    continue;
                }
                if i == j {
                    let turn = (j - i + 1) % 2;
                    if turn == player1 {
                        res[i][j] = (a[i - 1], 0);
                    } else {
                        res[i][j] = (0, a[i - 1]);
                    }
                    continue;
                }
                let turn = (j - i + 1) % 2;
                if turn == player1 {
                    let left_take = a[i - 1] + res[i + 1][j].0;
                    let right_take = a[j - 1] + res[i][j - 1].0;
                    if left_take > right_take {
                        res[i][j] = (left_take, res[i + 1][j].1);
                    } else {
                        res[i][j] = (right_take, res[i][j - 1].1);
                    }
                } else {
                    let left_take = a[i - 1] + res[i + 1][j].1;
                    let right_take = a[j - 1] + res[i][j - 1].1;
                    if left_take > right_take {
                        res[i][j] = (res[i + 1][j].0, left_take);
                    } else  {
                        res[i][j] = (res[i][j - 1].0, right_take);
                    }
                }
            }
        }

        // println!("{res:?}");

        res[1][n].0
    }
}

// @code end
