// Created by Ayush Biswas at 2025/07/17 21:06
// https://cses.fi/problemset/task/1163
use cp_lib::*;

// @code begin
use cpio::*;
use std::collections::{BTreeMap, BTreeSet, HashSet};

sol! {
    fn solution([x, n]: [usize; 2], a: [usize]) -> Words<usize> {
        a.into_iter()
            .scan(
                (
                    BTreeSet::from([(0, x)]),
                    BTreeMap::<usize, HashSet<(usize, usize)>>::new(),
                ),
                |(passage, lengths), ai| {
                    let &p = passage
                        .range(..(ai, ai))
                        .next_back()
                        .expect("Cannot find the passage to split");
                    passage.remove(&p);

                    let (s, e) = p;
                    let l = e - s;
                    lengths.entry(l).or_default().remove(&(s, e));
                    if lengths
                        .get(&l)
                        .expect("Cannot get but should be")
                        .is_empty()
                    {
                        lengths.remove(&l);
                    }

                    passage.insert((s, ai));
                    passage.insert((ai, e));
                    lengths.entry(ai - s).or_default().insert((s, ai));
                    lengths.entry(e - ai).or_default().insert((ai, e));

                    Some(*lengths.last_key_value().expect("Last key not found").0)
                },
            )
            .collect()
    }
}

// @code end
