// Created by Ayush Biswas at 2025/07/30 12:56
// https://cses.fi/problemset/task/1672
use cp_lib::*;

// @code begin
use cpio::*;
// const INF: u32 = 10u32.pow(9);
const INF: usize = 4 * 10usize.pow(18);

sol! {
    fn solution(
        [n, m, q]: [usize; 3],
        roads: [[usize; 3]; m],
        queries: [[usize; 2]; q]
    ) -> Lines<CPResult<usize, i8>> {
        let mut d = vec![vec![INF; n + 1]; n + 1];
        for [a, b, w] in roads {
            d[a][b] = d[a][b].min(w);
            d[b][a] = d[b][a].min(w);
        }
        for i in 1..=n {
            d[i][i] = 0;
        }
        for k in 1..=n {
            for i in 1..=n {
                for j in 1..=n {
                    d[i][j] = d[i][j].min(d[i][k] + d[k][j]);
                }
            }
        }
        queries.into_iter().map(|[a, b]| {
            if d[a][b] < INF {
                Success(d[a][b])
            } else {
                Failure(-1)
            }
        }).collect()
    }
}

// @code end
