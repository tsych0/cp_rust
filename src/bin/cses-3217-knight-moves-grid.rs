// Created by Ayush Biswas at 2025/07/12 14:36
// https://cses.fi/problemset/task/3217
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::VecDeque;

sol! {
    fn solution(
        n: usize,
    ) -> Lines<Words<usize>> {
        const MOVES: [(usize, usize); 2] = [(2, 1), (1, 2)];
        let mut board = vec![vec![usize::MAX; n]; n];
        let mut stack = VecDeque::new();
        stack.push_back((1, 0, 0));

        board[0][0] = 0;
        while !stack.is_empty() {
            let (depth, x, y) = stack.pop_front().unwrap();

            for (dx, dy) in MOVES {
                if x >= dx && y >= dy && board[x-dx][y-dy] > depth {
                    board[x - dx][y - dy] = depth;
                    stack.push_back((depth + 1, x - dx, y - dy));
                }
                if x + dx < n && y + dy < n && board[x+dx][y+dy] > depth {
                    board[x + dx][y + dy] = depth;
                    stack.push_back((depth + 1, x + dx, y + dy));
                }
                if x + dx < n && y >= dy && board[x+dx][y-dy] > depth {
                    board[x+dx][y-dy] = depth;
                    stack.push_back((depth + 1, x + dx, y - dy));
                }
                if x >= dx && y + dy < n && board[x-dx][y+dy] > depth {
                    board[x-dx][y+dy] = depth;
                    stack.push_back((depth + 1, x - dx, y + dy));
                }
            }
        }

        board.into_iter().map(words_of).collect()
    }
}

// @code end
