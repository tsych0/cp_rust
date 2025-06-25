use crate::indexmap::IndexSet;
use crate::internal_type_traits::Zero;
use std::{hash::Hash, ops::ControlFlow};

/// Compute a shortest path using the [IDA* search
/// algorithm](https://en.wikipedia.org/wiki/Iterative_deepening_A*).
pub fn idastar<N, C, FN, IN, FH, FS>(
    start: &N,
    mut successors: FN,
    mut heuristic: FH,
    mut success: FS,
) -> Option<(Vec<N>, C)>
where
    N: Eq + Clone + Hash,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    let mut path = IndexSet::from_iter([start.clone()].into_iter());

    std::iter::repeat(())
        .try_fold(heuristic(start), |bound, ()| {
            search(
                &mut path,
                Zero::zero(),
                bound,
                &mut successors,
                &mut heuristic,
                &mut success,
            )
            .map_break(Some)?
            // .filter(|min| *min > bound)
            .map_or(ControlFlow::Break(None), ControlFlow::Continue)
        })
        .break_value()
        .unwrap_or_default() // To avoid a missing panics section, as this always break
}

fn search<N, C, FN, IN, FH, FS>(
    path: &mut IndexSet<N>,
    cost: C,
    bound: C,
    successors: &mut FN,
    heuristic: &mut FH,
    success: &mut FS,
) -> ControlFlow<(Vec<N>, C), Option<C>>
where
    N: Eq + Clone + Hash,
    C: Zero + Ord + Copy,
    FN: FnMut(&N) -> IN,
    IN: IntoIterator<Item = (N, C)>,
    FH: FnMut(&N) -> C,
    FS: FnMut(&N) -> bool,
{
    let neighbs = {
        let start = path.get_index(path.len() - 1).unwrap();
        let f = cost + heuristic(start);
        if f > bound {
            return ControlFlow::Continue(Some(f));
        }
        if success(start) {
            return ControlFlow::Break((path.iter().cloned().collect(), f));
        }
        let mut neighbs: Vec<(N, C, C)> = successors(start)
            .into_iter()
            .filter_map(|(n, c)| {
                (!path.contains(&n)).then(|| {
                    let h = heuristic(&n);
                    (n, c, c + h)
                })
            })
            .collect::<Vec<_>>();
        neighbs.sort_unstable_by(|(_, _, c1), (_, _, c2)| c1.cmp(c2));
        neighbs
    };
    let mut min = None;
    for (node, extra, _) in neighbs {
        let (idx, _) = path.insert_full(node);
        match search(path, cost + extra, bound, successors, heuristic, success)? {
            Some(m) if min.is_none_or(|n| n >= m) => min = Some(m),
            _ => (),
        }
        path.swap_remove_index(idx);
    }
    ControlFlow::Continue(min)
}
