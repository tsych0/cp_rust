// Created by Ayush Biswas at 2025/07/10 21:04
// https://cses.fi/problemset/task/1624
use cp_lib::*;

// @code begin
use cpio::*;
use std::convert::TryInto;

sol! {
    fn solution(
        board: [[char]; 8]
    ) -> usize {
        queens(
            &board,
            0,
            &mut vec![8; 8].try_into().unwrap(),
            &mut vec![8; 15].try_into().unwrap(),
            &mut vec![8; 15].try_into().unwrap(),
        )
    }
}

fn queens(
    board: &Vec<Vec<char>>,
    row: usize,
    col: &mut [usize; 8],
    diag1: &mut [usize; 15],
    diag2: &mut [usize; 15],
) -> usize {
    if row == 8 {
        return 1;
    }
    let mut res = 0;
    for c in 0..8 {
        let d1 = row + c;
        let d2 = row + (7 - c);
        if board[row][c] == '.' && col[c] == 8 && diag1[d1] == 8 && diag2[d2] == 8 {
            col[c] = row;
            diag1[d1] = row;
            diag2[d2] = row;
            res += queens(board, row + 1, col, diag1, diag2);
            col[c] = 8;
            diag1[d1] = 8;
            diag2[d2] = 8;
        }
    }
    res
}

// @code end
