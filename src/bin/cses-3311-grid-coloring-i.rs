// Created by Ayush Biswas at 2025/07/13 00:13
// https://cses.fi/problemset/task/3311
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::HashSet;

const COLORS: [char; 4] = ['A', 'B', 'C', 'D'];

sol! {
    fn solution(
        [n, m]: [usize; 2],
        grid: [[char]; n]
    ) -> Lines<String> {
        let mut board = vec![vec!['x'; m]; n];
        coloring(&grid, &mut board, (0, 0), (n, m));
        board.into_iter().map(|v| v.into_iter().collect()).collect()
    }
}

fn coloring(
    grid: &Vec<Vec<char>>,
    board: &mut Vec<Vec<char>>,
    (x, y): (usize, usize),
    (n, m): (usize, usize),
) -> bool {
    if x >= n {
        return true;
    }

    let (xi, yi) = next_cell((x, y), (n, m));
    let mut taken = HashSet::new();
    if x > 0 {
        taken.insert(board[x - 1][y]);
    }
    if y > 0 {
        taken.insert(board[x][y - 1]);
    }

    for c in COLORS {
        if !taken.contains(&c) && grid[x][y] != c {
            board[x][y] = c;
            if coloring(grid, board, (xi, yi), (n, m)) {
                return true;
            }
        }
    }

    false
}

fn next_cell((x, y): (usize, usize), (n, m): (usize, usize)) -> (usize, usize) {
    let idx = x * m + y + 1;
    (idx / m, idx % m)
}

// @code end
