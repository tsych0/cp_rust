// Created by Ayush Biswas at 2025/07/22 13:32
// https://cses.fi/problemset/task/3314
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::collections::VecDeque;
use std::iter::once;

const INF: usize = 10usize.pow(9) + 7;

sol! {
    fn solution(
        n: usize,
        a: [usize]
    ) -> usize {
        let mut stack = Vec::with_capacity(n + 2);
        let a = once(INF).chain(a.into_iter())
                .chain(once(INF))
                .collect_vec();

        let mut graph = vec![vec![]; n + 2];

        stack.push(0);
        for i in 1..n+1 {
            while a[i] >= a[stack[stack.len() - 1]] {
                stack.pop();
            }
            graph[stack[stack.len() - 1]].push(i);
            stack.push(i);
        }

        stack.push(n + 1);
        for i in (1..n+1).rev() {
            while a[i] >= a[stack[stack.len() - 1]] {
                stack.pop();
            }
            graph[stack[stack.len() - 1]].push(i);
            stack.push(i);
        }

        let mut res = vec![0; n + 2];

        let mut queue = VecDeque::with_capacity(n + 2);
        let mut count = vec![0; n + 2];

        queue.push_back(0);
        queue.push_back(n + 1);

        while !queue.is_empty() {
            let pos = queue.pop_front().unwrap();
            for &child in &graph[pos] {
                res[child] = res[child].max(res[pos] + 1);
                count[child] += 1;
                if count[child] == 2 { // this means this is the max of this segment
                    queue.push_back(child);
                }
            }
        }

        *res.iter().max().unwrap()
    }
}

// @code end
