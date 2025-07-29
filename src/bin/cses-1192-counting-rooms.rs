// Created by Ayush Biswas at 2025/07/25 15:06
// https://cses.fi/problemset/task/1192
use cp_lib::*;

// @code begin
use cpio::*;
const MOVES: [fn(usize, usize) -> (usize, usize); 4] = [
    |x: usize, y: usize| (x - 1, y),
    |x: usize, y: usize| (x, y - 1),
    |x: usize, y: usize| (x + 1, y),
    |x: usize, y: usize| (x, y + 1),
];

sol! {
    fn solution(
        [n, m]: [usize; 2],
        map: [[char]; n]
    ) -> usize {

        let mut visited = vec![vec![false; m]; n];
        fn dfs(map: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize,  n: usize, m: usize) {
            for mv in MOVES {
                let (x, y) = mv(i, j);
                if x < n && y < m && !visited[x][y] && map[x][y] == '.' {
                    visited[x][y] = true;
                    dfs(map, visited, x, y, n, m);
                }
            }
        }

        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                if !visited[i][j] && map[i][j] == '.' {
                    dfs(&map, &mut visited, i, j, n, m);
                    res += 1;
                }
            }
        }
        res
    }
}

// @code end
