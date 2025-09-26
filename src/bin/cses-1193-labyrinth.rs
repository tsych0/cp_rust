// Created by Ayush Biswas at 2025/07/25 19:27
// https://cses.fi/problemset/task/1193
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::VecDeque;

sol! {
    fn solution(
        [n, m]: [usize; 2],
        map: [[char]; n]
    ) -> CPResult<String, String> {
        let start_point = find_start(&map, n, m);
        let (end_point, directions) =
            unwrap!(find_directions(map, start_point, n, m).ok_or("No".into()));
        let mut res = vec![];
        let (mut i, mut j) = end_point;
        while (i, j) != start_point {
            res.push(directions[i][j]);
            (i, j) = go(opposite_direction(directions[i][j]), i, j);
        }
        let path: String = res.into_iter().rev().collect();
        CPResult::Success(format!("YES\n{}\n{}", path.len(), path))
    }
}

fn find_directions(
    map: Vec<Vec<char>>,
    start: (usize, usize),
    n: usize,
    m: usize,
) -> Option<((usize, usize), Vec<Vec<char>>)> {
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![false; m]; n];
    let mut direction = vec![vec![' '; m]; n];
    queue.push_back(start);
    while let Some((i, j)) = queue.pop_front() {
        if map[i][j] == 'B' {
            return Some(((i, j), direction));
        }
        for mv in ['U', 'D', 'L', 'R'] {
            let (x, y) = go(mv, i, j);
            if x < n && y < m && !visited[x][y] && map[x][y] != '#' {
                visited[x][y] = true;
                direction[x][y] = mv;
                queue.push_back((x, y));
            }
        }
    }
    None
}

fn find_start(map: &Vec<Vec<char>>, n: usize, m: usize) -> (usize, usize) {
    for i in 0..n {
        for j in 0..m {
            if map[i][j] == 'A' {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn go(d: char, i: usize, j: usize) -> (usize, usize) {
    match d {
        'U' => (i - 1, j),
        'D' => (i + 1, j),
        'L' => (i, j - 1),
        'R' => (i, j + 1),
        _ => (i, j),
    }
}

fn opposite_direction(d: char) -> char {
    match d {
        'U' => 'D',
        'L' => 'R',
        'D' => 'U',
        'R' => 'L',
        d => d,
    }
}

// @code end
