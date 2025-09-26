// Created by Ayush Biswas at 2025/07/26 11:01
// https://cses.fi/problemset/task/1668
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::VecDeque;

sol! {
    fn solution(
        [n, m]: [usize; 2],
        friendships: [[usize; 2]; m]
    ) -> CPResult<Words<usize>, String> {
        let mut friends = vec![vec![]; n + 1];
        for [a, b] in friendships {
            friends[a].push(b);
            friends[b].push(a);
        }
        let grps =
            unwrap!(assign_grps(friends, n).ok_or("IMPOSSIBLE".to_string()));
        Success(grps.into())
    }
}

fn assign_grps(friends: Vec<Vec<usize>>, n: usize) -> Option<Vec<usize>> {
    let mut grp = vec![0; n + 1];
    for i in 1..=n {
        if grp[i] != 0 {
            continue;
        }
        grp[i] = 1;
        let mut queue = VecDeque::new();
        queue.push_back(i);
        while let Some(p) = queue.pop_front() {
            let next_grp = 3 - grp[p];
            for &friend in &friends[p] {
                if grp[friend] == grp[p] {
                    return None;
                }
                if grp[friend] == 0 {
                    queue.push_back(friend);
                    grp[friend] = next_grp;
                }
            }
        }
    }
    Some(grp[1..].to_vec())
}

// @code end
