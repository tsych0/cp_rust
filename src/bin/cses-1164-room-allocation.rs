// Created by Ayush Biswas at 2025/07/18 11:12
// https://cses.fi/problemset/task/1164
use cp_lib::*;

// @code begin
use cpio::*;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, BinaryHeap},
    ops::AddAssign,
};

sol! {
    fn solution(n: usize, stays: [[usize; 2]; n]) -> Lines<Words<usize>> {
        let res = stays
            .into_iter()
            .zip(0..)
            .sorted()
            .scan(
                (BTreeMap::<usize, BinaryHeap<usize>>::new(), 1),
                |(bookings, next_room), ([arrival, departure], idx)| {
                    if let Some((&dep, rooms)) = bookings.range_mut(..arrival).next() {
                        let room = rooms.pop().unwrap();
                        if rooms.is_empty() {
                            bookings.remove(&dep);
                        }
                        bookings.entry(departure).or_default().push(room);
                        Some((idx, room))
                    } else {
                        let room = *next_room;
                        next_room.add_assign(1);
                        bookings.entry(departure).or_default().push(room);
                        Some((idx, room))
                    }
                },
            )
            .sorted()
            .map(|(_, r)| r)
            .collect_vec();
        lines_of(vec![vec![*res.iter().max().unwrap()].into(), res.into()])
    }
}

// @code end
