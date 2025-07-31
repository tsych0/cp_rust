// Created by Ayush Biswas at 2025/07/31 20:14
// https://cses.fi/problemset/task/1195
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::BinaryHeap;
// const INF: u32 = 10u32.pow(9);
const INF: isize = 4 * 10isize.pow(18);

#[derive(Clone)]
struct Edge {
    child: usize,
    cost: isize,
}

sol! {
    fn solution(
        [n, m]: [usize; 2],
        flights: [[usize; 3]; m]
    ) -> isize {
        let mut graph_forwards = vec![vec![]; n + 1];
        let mut graph_backwards = vec![vec![]; n + 1];

        for &[a, b, c] in &flights {
            graph_forwards[a].push(Edge {child: b, cost: c as isize});
            graph_backwards[b].push(Edge {child: a, cost: c as isize});
        }

        let d_forwards = dijkstra(graph_forwards, 1, n);
        let d_backwards = dijkstra(graph_backwards, n, n);

        let mut res = INF;
        for [a, b, c] in flights {
            res = res.min(d_forwards[a] + d_backwards[b] + c as isize / 2);
        }

        res
    }
}

fn dijkstra(graph: Vec<Vec<Edge>>, start: usize, n: usize) -> Vec<isize> {
    let mut d = vec![INF; n + 1];
    let mut heap = BinaryHeap::new();

    d[start] = 0;
    heap.push((0isize, start));

    while let Some((dist, node)) = heap.pop() {
        if -dist > d[node] {
            continue;
        }
        for &Edge { child, cost } in &graph[node] {
            if cost + d[node] < d[child] {
                d[child] = cost + d[node];
                heap.push((-d[child], child));
            }
        }
    }

    d
}

// @code end
