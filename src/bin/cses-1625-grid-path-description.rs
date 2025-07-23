// Created by Ayush Biswas at 2025/07/13 17:56
// https://cses.fi/problemset/task/1625
use cp_lib::*;

// @code begin
use cpio::*;

static mut VISITED: [[bool; 7]; 7] = [
    [false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false],
    [false, false, false, false, false, false, false],
];

static mut S: [char; 48] = [
    '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?',
    '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?', '?',
    '?', '?', '?', '?', '?', '?', '?', '?', '?', '?',
];

sol! {
    fn solution(
        s: [char]
    ) -> usize {
        for (i, c) in s.into_iter().enumerate() {
            unsafe {
                S[i] = c;
            }
        }
        paths((0, 0), 0)
    }
}

const MOVES: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn paths((i, j): (isize, isize), depth: usize) -> usize {
    if i == 6 && j == 0 && depth == 48 {
        return 1;
    }

    if depth >= 48 {
        return 0;
    }
    if (blocked(i + 1, j) && blocked(i - 1, j) && !blocked(i, j + 1) && !blocked(i, j - 1))
        || (blocked(i, j + 1) && blocked(i, j - 1) && !blocked(i + 1, j) && !blocked(i - 1, j))
    {
        return 0;
    }

    unsafe {
        VISITED[i as usize][j as usize] = true;
    }

    let mut res = 0;

    if unsafe { S }[depth] != '?' {
        let (dx, dy) = mv(unsafe { S }[depth]);
        if !blocked(i + dx, j + dy) {
            res += paths((i + dx, j + dy), depth + 1);
        }
    } else {
        for (dx, dy) in MOVES {
            if !blocked(i + dx, j + dy) {
                res += paths((i + dx, j + dy), depth + 1);
            }
        }
    }

    unsafe {
        VISITED[i as usize][j as usize] = false;
    }

    res
}

fn blocked(i: isize, j: isize) -> bool {
    unsafe { return i < 0 || j < 0 || i >= 7 || j >= 7 || VISITED[i as usize][j as usize] }
}

fn mv(c: char) -> (isize, isize) {
    match c {
        'D' => (1, 0),
        'U' => (-1, 0),
        'R' => (0, 1),
        'L' => (0, -1),
        _ => (0, 0),
    }
}

// @code end
