use crate::indexmap::{
    Entry::{Occupied, Vacant},
    IndexMap,
};
use crate::internal_type_traits::{Bounded, Zero};
use crate::pathfinding::reverse_path;
use std::collections::VecDeque;
use std::hash::Hash;
use std::mem;

/// Compute a shortest path using the [Fringe search algorithm].
#[expect(clippy::missing_panics_doc)]
pub fn fringe<N, C, FN, IN, FH, FS>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Hash + Clone,
    C: Bounded + Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool, {
    let mut now = VecDeque::new();
    let mut later = VecDeque::new();
    let mut parents: IndexMap<N, (usize, C)> = IndexMap::new();
    let mut flimit = heuristic(start);
    now.push_back(0);
    parents.insert(start.clone(), (usize::MAX, Zero::zero()));

    loop {
        if now.is_empty() {
            return None;
        }
        let mut fmin = C::max_value();
        while let Some(i) = now.pop_front() {
            let (g, successors) = {
                let (node, &(_, g)) = parents.get_index(i).unwrap(); // Cannot fail
                let f = g + heuristic(node);
                if f > flimit {
                    if f < fmin {
                        fmin = f;
                    }
                    later.push_back(i);
                    continue;
                }
                if success(node) {
                    let path = reverse_path(&parents, |&(p, _)| p, i);
                    return Some((path, g));
                }
                (g, successors(node))
            };
            for (successor, cost) in successors {
                let g_successor = g + cost;
                let n; // index for successor
                match parents.entry(successor) {
                    Vacant(e) => {
                        n = e.index();
                        e.insert((i, g_successor));
                    }
                    Occupied(mut e) => {
                        if e.get().1 > g_successor {
                            n = e.index();
                            e.insert((i, g_successor));
                        } else {
                            continue;
                        }
                    }
                }
                if !remove(&mut later, &n) {
                    remove(&mut now, &n);
                }
                now.push_front(n);
            }
        }
        mem::swap(&mut now, &mut later);
        flimit = fmin;
    }
}

fn remove<T: Eq>(v: &mut VecDeque<T>, e: &T) -> bool {
    v.iter().position(|x| x == e).is_some_and(|index| {
        v.remove(index);
        true
    })
}
