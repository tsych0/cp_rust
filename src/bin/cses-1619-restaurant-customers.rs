// Created by Ayush Biswas at 2025/07/15 16:23
// https://cses.fi/problemset/task/1619
use cp_lib::*;

// @code begin
use cpio::*;

sol! {
    fn solution(
        n: usize,
        a: [[usize; 2]; n]
    ) -> usize {
        let (mut arriving, mut leaving): (Vec<_>, Vec<_>) = a.iter().map(|&[a, l]| (a, l)).unzip();
        arriving.sort();
        leaving.sort();

        let mut res = 0;
        // so basically at each incident
        // either someone is leaving or entering
        // we are checking what is the current number of people
        // in the restaurant.
        for a in a.into_iter().flatten() {
            let arrived_till_this = arriving.partition_point(|&ai| ai <= a);
            let left_before_this = leaving.partition_point(|&li| li < a);
            res = res.max(arrived_till_this.saturating_sub(left_before_this));
        }
        res
    }
}

// @code end
