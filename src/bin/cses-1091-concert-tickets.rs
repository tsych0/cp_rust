// Created by Ayush Biswas at 2025/07/15 15:33
// https://cses.fi/problemset/task/1091
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::{collections::BTreeMap, ops::SubAssign};

sol! {
    fn solution([n, m]: [usize; 2], ticket_price: [usize], budgets: [usize]) -> Lines<Result<usize>> {
        let mut prices: BTreeMap<usize, usize> = ticket_price
            .into_iter()
            .sorted()
            .group_by(|&p| p)
            .map(|g| (g[0], g.len()))
            .collect();

        budgets
            .into_iter()
            .map(|budget| {
                if let Some((&price, count)) = prices.range_mut(..=budget).next_back() {
                    if *count == 1 {
                        prices.remove(&price);
                    } else {
                        count.sub_assign(1);
                    }
                    Ok(price)
                } else {
                    Err("-1".into())
                }
            })
            .collect::<Vec<_>>()
            .into()
    }
}

// @code end
