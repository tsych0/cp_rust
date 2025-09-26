// Created by Ayush Biswas at 2025/07/26 11:19
// https://cses.fi/problemset/task/1669
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::VecDeque;

sol! {
    fn solution(
        [n, m]: [usize; 2],
        roads: [[usize; 2]; m]
    ) -> CPResult<Lines<Words<usize>>, String> {
        let mut graph = vec![vec![]; n + 1];
        for [a, b] in roads {
            graph[a].push(b);
            graph[b].push(a);
        }
        let (p, parents) =
            unwrap!(find_cycle(graph, n).ok_or("IMPOSSIBLE".to_string()));
        // println!("{parents:?}");
        let mut res = vec![p];
        let mut q = p;
        loop {
            q = parents[q];
            res.push(q);
            if q == p {
                break;
            }
        }
        CPResult::Success(vec![
            words_of(vec![res.len()]),
            words_of(res),
        ].into())
    }
}

fn find_cycle(graph: Vec<Vec<usize>>, n: usize) -> Option<(usize, Vec<usize>)> {
    let mut parent = vec![0; n + 1];
    for i in 1..=n {
        if parent[i] != 0 {
            continue;
        }
        let mut stack = VecDeque::new();
        stack.push_back((i, i));
        while let Some((city, p)) = stack.pop_back() {
            parent[city] = p;
            // println!("city: {city}");
            for &neighbor in &graph[city] {
                if parent[city] == neighbor {
                    continue;
                }
                if parent[neighbor] != 0 {
                    // println!("neighbor = {neighbor}, parent = {city}");
                    parent[neighbor] = city;
                    return Some((neighbor, parent));
                }
                // println!("neighbor: {neighbor}");
                stack.push_back((neighbor, city));
            }
        }
    }
    None
}

// @code end
