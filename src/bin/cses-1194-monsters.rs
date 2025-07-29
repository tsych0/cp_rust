// Created by Ayush Biswas at 2025/07/27 20:47
// https://cses.fi/problemset/task/1194
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::VecDeque;
// const INF: u32 = 10u32.pow(9);
const INF: usize = 4 * 10usize.pow(18);

sol! {
    fn solution(
        [n, m]: [usize; 2],
        (mut grid): [[char]; n]
    ) -> CPResult<Lines<String>, String> {
        let mut queue = VecDeque::new(); 
        queue.extend(find_monsters(&grid, n, m).into_iter().map(|(x, y)| ('M', x, y)));
        let (startx, starty) = find_start(&grid, n, m);
        queue.push_back(('A', startx, starty));
        
        let mut direction = vec![vec![' '; m]; n];
        let mut res: Option<(usize, usize)> = None;
        while let Some((p, i, j)) = queue.pop_front() {
            if (i == n - 1 || j == m - 1 || i == 0 || j == 0) && p == 'A' {
                _ = res.insert((i, j));
                break;
            }
            let moves = ['U', 'D', 'L', 'R'];
            if p == 'A' {
                for mv in moves {
                    let (x, y) = go(mv, i, j);
                    if x < n && y < m && grid[x][y] == '.' {
                        grid[x][y] = 'A';
                        direction[x][y] = mv;
                        queue.push_back(('A', x, y));
                    }
                }
            } else {
                for mv in moves {
                    let (x, y) = go(mv, i, j);
                    if x < n && y < m && grid[x][y] != '#' && grid[x][y] != 'M' {
                        grid[x][y] = 'M';
                        queue.push_back(('M', x, y));
                    }
                } 
            }
        }
        
        let (mut i, mut j) = unwrap!(res.ok_or("No".into()));
        
        let mut result = vec![];
        loop {
            let d = direction[i][j];
            if d == ' ' {
                break;
            }
            result.push(d);
            (i, j) = go(opposite_direction(d), i, j); 
        }
        Success(
            vec![
                "YES".to_string(),
                result.len().to_string(),
                result.into_iter().rev().collect()
            ].into()
        )
    }
}

fn find_monsters(grid: &Vec<Vec<char>>, n: usize, m: usize) -> Vec<(usize, usize)> {
    let mut res = vec![];
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'M' {
                res.push((i, j));
            }
        }
    }
    res
}

fn find_start(grid: &Vec<Vec<char>>, n: usize, m: usize) -> (usize, usize) {
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'A' {
                return (i, j)
            }
        }
    }
    (INF, INF)
}

fn go(d: char, i: usize, j: usize) -> (usize, usize) {
    match d {
        'U' => (i - 1, j),
        'D' => (i + 1, j),
        'L' => (i, j - 1),
        'R' => (i, j + 1),
        _ => (i, j)
    }
}

fn opposite_direction(d: char) -> char {
    match d {
        'U' => 'D',
        'L' => 'R',
        'D' => 'U',
        'R' => 'L',
        d => d
    }
}

// @code end
