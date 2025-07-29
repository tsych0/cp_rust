// Created by Ayush Biswas at 2025/07/25 23:29
// https://cses.fi/problemset/task/1667
use cp_lib::*;

// @code begin
use std::collections::VecDeque;
use itertools::Itertools;
use cpio::*;

sol! {
    fn solution(
        [n, m]: [usize; 2],
        connections: [[usize; 2]; m]
    ) -> CPResult<Lines<Words<usize>>, String> {
        let mut graph = vec![vec![]; n + 1];
        for [a, b] in connections {
            graph[a].push(b);
            graph[b].push(a);
        }
        
        // Calculating the parents for the shortest path using bfs
        let mut parent = vec![0; n + 1];
        let mut queue = VecDeque::new(); 
        queue.push_back(1);
        parent[1] = n + 1;
        while let Some(node) = queue.pop_front() {
            if node == n {
                break;
            }
            for &child in &graph[node] {
                if parent[child] == 0 {
                    parent[child] = node;
                    queue.push_back(child);
                } 
            }
        }
        
        // Building the result
        let res = (1..).scan(n, |state, _| {
            if (*state != n + 1) && (*state != 0) {
                let curr = *state;
                *state = parent[curr];
                Some(curr)
            } else {
                None
            }
        }).collect_vec();
        
        if res.len() == 1 {
            Failure("IMPOSSIBLE".into())
        } else {
            Success(vec![
                vec![res.len()].into(),
                res.into_iter().rev().collect()
            ].into())
        }
    }
}

// @code end
