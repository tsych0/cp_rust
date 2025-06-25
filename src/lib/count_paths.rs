//! Count the total number of possible paths to reach a destination.

use std::collections::HashMap;
use std::hash::Hash;

fn cached_count_paths<T, FN, IN, FS>(
    start: T,
    successors: &mut FN,
    success: &mut FS,
    cache: &mut HashMap<T, usize>,
) -> usize
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
{
    if let Some(&n) = cache.get(&start) {
        return n;
    }

    let count = if success(&start) {
        1
    } else {
        successors(&start)
            .into_iter()
            .map(|successor| cached_count_paths(successor, successors, success, cache))
            .sum()
    };

    cache.insert(start, count);

    count
}

/// Count the total number of possible paths to reach a destination. There must be no loops
/// is the graph, or the function will overflow its stack.
pub fn count_paths<T, FN, IN, FS>(start: T, mut successors: FN, mut success: FS) -> usize
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
{
    cached_count_paths(
        start,
        &mut successors,
        &mut success,
        &mut HashMap::default(),
    )
}
