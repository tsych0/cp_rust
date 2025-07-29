// Created by Ayush Biswas at 2025/07/28 08:08
// https://cses.fi/problemset/task/1671
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::BinaryHeap;
// const INF: u32 = 10u32.pow(9);
const INF: isize = 4 * 10isize.pow(18);

sol! {
    fn solution(
        [n, m]: [usize; 2],
        flights: [[usize; 3]; m]
    ) -> Words<isize> {
        let mut graph = vec![vec![]; n + 1];
        for [a, b, c] in flights {
            graph[a].push((b, c as isize));
        }
        let mut distances = vec![INF; n + 1];

        let mut heap = BinaryHeap::new();

        distances[1] = 0;
        heap.push((0, 1));

        while let Some((dist, parent)) = heap.pop() {
            if -dist > distances[parent] {
                continue;
            }
            for &(child, w) in &graph[parent] {
                if distances[parent] + w < distances[child] {
                    distances[child] = distances[parent] + w;
                    heap.push((-distances[child], child));
                }
            }
        }

        distances.drain(1..).collect()
    }
}
// @code end
